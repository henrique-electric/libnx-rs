"""Evaluate the object-like #defines of the libnx headers on aarch64 and write macros.json.

macros.json: { "switch/services/hid.h": [[name, rust_type, rust_value, doc_lines], ...], ... }
"""
import json
import os
import re
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SCRATCH = os.path.join(ROOT, "target", "libnx-gen")  # clang outputs (see regen.sh)
INC = os.path.join(ROOT, "include") + "/"
CC = os.path.join(ROOT, "tools", "cc.sh")

# conv_doc is shared with the generator; import it without running gen.py's main body.
_src = open(os.path.join(ROOT, "tools", "gen.py")).read()
_ns = {"re": re}
exec(_src[_src.index("def conv_inline"):_src.index("def doc_for")], _ns)
conv_doc = _ns["conv_doc"]


def candidates():
    cur = None
    out = []
    seen = set()
    for line in open(os.path.join(SCRATCH, "macros.i"), errors="replace"):
        m = re.match(r'# \d+ "([^"]+)"', line)
        if m:
            cur = os.path.normpath(m.group(1))
            continue
        m = re.match(r"#define (\w+)(\(?)\s*(.*)$", line.rstrip("\n"))
        if not m or not cur or not cur.startswith(INC):
            continue
        name, fnlike, body = m.groups()
        if fnlike or not body.strip() or name in seen:
            continue
        if body.strip().startswith("__attribute__") or re.match(r"_\w+_H_?$", name):
            continue
        seen.add(name)
        out.append((name, cur[len(INC):], body.strip()))
    return out


KIND = ("_Generic((X), int:1, unsigned int:2, long:3, unsigned long:4, long long:5, unsigned long long:6, "
        "char*:7, const char*:7, double:9, float:10, _Bool:11, unsigned char:12, signed char:13, char:13, "
        "short:14, unsigned short:15, default:0)")


def gen_c(cands):
    lines = ['#include "' + os.path.join(SCRATCH, "all.c") + '"']
    spans = []
    for name, rel, _ in cands:
        start = len(lines) + 1
        x = f"({name})"
        lines.append(f"const int __k_{name} = {KIND.replace('X', name)};")
        lines.append(f"const unsigned long long __v_{name} = _Generic({x}, char*: 0ULL, const char*: 0ULL, "
                     f"double: 0ULL, float: 0ULL, default: (unsigned long long){x});")
        lines.append(f"const char* const __s_{name} = _Generic({x}, char*: {x}, const char*: {x}, default: \"\");")
        lines.append(f"const double __d_{name} = _Generic({x}, double: {x}, float: {x}, default: 0.0);")
        spans.append((start, len(lines), name))
    return "\n".join(lines) + "\n", spans


def evaluate(cands):
    work = SCRATCH
    os.makedirs(work, exist_ok=True)
    cands = list(cands)
    while True:
        src, spans = gen_c(cands)
        cpath = os.path.join(work, "macros_eval.c")
        open(cpath, "w").write(src)
        r = subprocess.run([CC, "-S", "-emit-llvm", "-O0", "-w", "-ferror-limit=0", "-o",
                            os.path.join(work, "macros_eval.ll"), cpath], capture_output=True, text=True)
        if r.returncode == 0:
            break
        bad = set()
        for m in re.finditer(r"macros_eval\.c:(\d+):\d+: error", r.stderr):
            ln = int(m.group(1))
            for s, e, n in spans:
                if s <= ln <= e:
                    bad.add(n)
        if not bad:
            print(r.stderr[:3000])
            raise SystemExit("could not attribute errors")
        cands = [c for c in cands if c[0] not in bad]
    ir = open(os.path.join(work, "macros_eval.ll")).read()
    strs = {m.group(1): m.group(2) for m in re.finditer(r'^(@[\w.]+) = .*?constant \[\d+ x i8\] c"(.*)"', ir, re.M)}
    vals = {}
    for m in re.finditer(r"^@__(k|v|s|d)_(\w+) = (?:dso_local )?constant (\w+) (.+?)(?:, align \d+)?$", ir, re.M):
        kind, name, ty, v = m.groups()
        vals.setdefault(name, {})[kind] = v.strip()
    return cands, vals, strs


def c_string(s):
    out = bytearray()
    i = 0
    while i < len(s):
        if s[i] == "\\":
            out.append(int(s[i + 1:i + 3], 16))
            i += 3
        else:
            out.append(ord(s[i]))
            i += 1
    return bytes(out.rstrip(b"\0"))


def rust_str(b):
    r = ""
    for ch in b:
        c = chr(ch)
        if c in "\\\"":
            r += "\\" + c
        elif 32 <= ch < 127:
            r += c
        else:
            r += f"\\x{ch:02x}"
    return r


def to_rust(kind, v, s, d, strs):
    if kind in (1, 3, 5, 13, 14):  # signed
        bits = {1: 32, 3: 64, 5: 64, 13: 8, 14: 16}[kind]
        n = v if v < 2 ** 63 else v - 2 ** 64
        n &= (1 << bits) - 1
        if n >= 1 << (bits - 1):
            n -= 1 << bits
        if n >= 0:
            ty = {32: "u32", 64: "u64", 8: "u8", 16: "u16"}[bits]
        else:
            ty = {32: "i32", 64: "i64", 8: "i8", 16: "i16"}[bits]
        return ty, n
    if kind in (2, 4, 6, 12, 15):
        bits = {2: 32, 4: 64, 6: 64, 12: 8, 15: 16}[kind]
        return {32: "u32", 64: "u64", 8: "u8", 16: "u16"}[bits], v & ((1 << bits) - 1)
    if kind == 11:
        return "bool", "true" if v else "false"
    if kind == 7:
        return "&CStr", f'c"{rust_str(c_string(strs.get(s, "")))}"'
    if kind in (9, 10):
        f = float.fromhex(d) if d.startswith("0x") else float(d)
        return ("f64" if kind == 9 else "f32"), repr(f)
    return None


def fmt(ty, n, body):
    if not isinstance(n, int):
        return str(n)
    if re.fullmatch(r"\(?\s*0[xX][0-9a-fA-F]+[uUlL]*\s*\)?", body) or (abs(n) >= 4096 and not re.fullmatch(r"\d+[uUlL]*", body)):
        return hex(n) if n >= 0 else "-" + hex(-n)
    return str(n)


_src_cache = {}


def macro_doc(rel, name):
    path = INC + rel
    lines = _src_cache.setdefault(path, open(path, errors="replace").read().splitlines())
    for i, l in enumerate(lines):
        if re.match(r"\s*#\s*define\s+" + re.escape(name) + r"\b", l):
            m = re.search(r"(///<|//!<|/\*\*<|/\*!<)(.*?)(\*/)?\s*$", l)
            if m:
                return conv_doc([m.group(2)])
            block = []
            j = i - 1
            if j >= 0 and lines[j].strip().endswith("*/"):
                while j >= 0:
                    block.insert(0, lines[j])
                    if re.search(r"/\*\*|/\*!", lines[j]):
                        break
                    if lines[j].strip().startswith("/*") and not lines[j].strip().startswith(("/**", "/*!")):
                        return []
                    j -= 1
                return conv_doc(block)
            while j >= 0 and lines[j].strip().startswith("///"):
                block.insert(0, lines[j])
                j -= 1
            return conv_doc(block)
    return []


def main():
    cands = candidates()
    print("candidates:", len(cands))
    cands, vals, strs = evaluate(cands)
    out = {}
    skipped = 0
    for name, rel, body in cands:
        v = vals.get(name)
        if not v:
            skipped += 1
            continue
        kind = int(v["k"])
        if kind == 0:
            skipped += 1
            continue
        num = int(v.get("v", "0")) if v.get("v", "0").lstrip("-").isdigit() else 0
        if num < 0:
            num += 2 ** 64
        r = to_rust(kind, num, v.get("s", "").split()[-1] if v.get("s") else "", v.get("d", "0.0"), strs)
        if r is None:
            skipped += 1
            continue
        ty, val = r
        out.setdefault(rel, []).append([name, ty, fmt(ty, val, body), macro_doc(rel, name)])
    json.dump(out, open(os.path.join(SCRATCH, "macros.json"), "w"), indent=0)
    print("constants:", sum(len(v) for v in out.values()), "skipped:", skipped)


main()
