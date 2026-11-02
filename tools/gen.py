"""Generate Rust modules for the libnx headers from clang's JSON AST + record layouts.

Inputs (produced by ../cc.sh):  ast.json, layouts.txt, macros.json (see macros.py)
Hand-written inline ports live in <crate>/inline_ports/<module path>.rs and are spliced in.
"""
import json
import os
import re
import sys
from collections import defaultdict, OrderedDict

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SCRATCH = os.path.join(ROOT, "target", "libnx-gen")  # clang outputs (see regen.sh)
INC = os.path.join(ROOT, "include") + "/"
CRATE = ROOT
OUT = os.path.join(CRATE, "src")

# Headers already ported by hand (module stays as-is).
HANDWRITTEN = {
    "switch/types.h", "switch/result.h", "switch/arm/tls.h", "switch/arm/counter.h", "switch/arm/cache.h",
    "switch/arm/thread_context.h", "switch/kernel/svc.h", "switch/sf/hipc.h", "switch/sf/cmif.h",
    "switch/sf/service.h", "switch/sf/tipc.h", "switch/services/sm.h",
}
# Umbrella / alias headers that declare nothing themselves.
SKIP_HEADERS = {"switch.h"}

RUST_KEYWORDS = set("""as break const continue crate else enum extern false fn for if impl in let loop match mod move
mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn abstract
become box do final macro override priv typeof unsized virtual yield try gen union""".split())

# ---------------------------------------------------------------------------------------------- helpers

def ident(name):
    return name + "_" if name in RUST_KEYWORDS else name


def header_to_module(rel):
    """'switch/services/hid.h' -> ['services', 'hid'];  'net/if.h' -> ['bsd', 'net', 'if_']"""
    parts = rel[:-2].split("/")
    if parts[0] == "switch":
        parts = parts[1:]
    else:
        parts = ["bsd"] + parts
    return [ident(p) for p in parts]


warnings = []


def warn(msg):
    warnings.append(msg)


# ---------------------------------------------------------------------------------------------- load AST

ast = json.load(open(os.path.join(SCRATCH, "ast.json")))

_cur = {"file": None, "line": None}


def stamp(node):
    """Walk the JSON in document order, resolving the elided file/line of every source location."""
    if isinstance(node, dict):
        if "offset" in node or "spellingLoc" in node or "expansionLoc" in node:
            if "spellingLoc" in node:
                stamp(node["spellingLoc"])
            if "expansionLoc" in node:
                stamp(node["expansionLoc"])
            if "offset" in node:
                if "file" in node:
                    _cur["file"] = node["file"]
                if "line" in node:
                    _cur["line"] = node["line"]
                node["_file"], node["_line"] = _cur["file"], _cur["line"]
            return
        for v in node.values():
            stamp(v)
    elif isinstance(node, list):
        for v in node:
            stamp(v)


stamp(ast)


def loc_of(n, key="loc"):
    l = n.get(key) or {}
    if "expansionLoc" in l:
        l = l["expansionLoc"]
    return l.get("_file"), l.get("_line"), l.get("col")


def rel_of(path):
    path = path and os.path.normpath(path)
    return path[len(INC):] if path and path.startswith(INC) else None


# ---------------------------------------------------------------------------------------------- record layouts

layouts = {}  # key -> {"size","align","fields":[(name, bitoff, bitwidth or None)]}


def layout_key_from_header(h):
    m = ANON_RE.search(h)
    if m:
        return "anon:" + norm_loc(m.group(1).replace(" ", ""))
    m = re.match(r"(struct|union) (\S+)$", h.strip())
    return f"{m.group(1)}:{m.group(2)}" if m else None


def parse_layouts():
    text = open(os.path.join(SCRATCH, "layouts.txt")).read()
    for block in text.split("*** Dumping AST Record Layout")[1:]:
        lines = [l for l in block.splitlines() if "|" in l]
        head = lines[0].split("|", 1)[1]
        key = layout_key_from_header(head)
        fields = []
        size = align = None
        has_union = head.strip().startswith("union")
        for l in lines[1:]:
            if re.match(r"\s*(const |volatile )*union ", l.split("|", 1)[1]):
                has_union = True
            off, rest = l.split("|", 1)
            m = re.match(r"\s*\[sizeof=(\d+), align=(\d+)", rest)
            if m:
                size, align = int(m.group(1)), int(m.group(2))
                continue
            depth = len(rest) - len(rest.lstrip(" "))
            if depth != 3:
                continue
            off = off.strip()
            fname = rest.strip().split(" ")[-1]
            if ":" in off:
                byte, bits = off.split(":")
                lo, hi = bits.split("-")
                fields.append((fname, int(byte) * 8 + int(lo), int(hi) - int(lo) + 1))
            else:
                fields.append((fname, int(off) * 8, None))
        layouts[key] = {"size": size, "align": align, "fields": fields, "has_union": has_union}


# ---------------------------------------------------------------------------------------------- C type parsing

PRIMS = {
    "void": "c_void", "char": "c_char", "signed char": "c_schar", "unsigned char": "c_uchar",
    "short": "c_short", "unsigned short": "c_ushort", "int": "c_int", "unsigned int": "c_uint",
    "long": "c_long", "unsigned long": "c_ulong", "long long": "c_longlong", "unsigned long long": "c_ulonglong",
    "float": "f32", "double": "f64", "_Bool": "bool", "bool": "bool", "long double": "u128",
    "__int128": "i128", "unsigned __int128": "u128", "__uint128_t": "u128", "__int128_t": "i128",
    "short int": "c_short", "long int": "c_long", "unsigned short int": "c_ushort", "long unsigned int": "c_ulong",
    "signed": "c_int", "unsigned": "c_uint",
}
# typedef names rendered straight to Rust types (no alias emitted)
DIRECT = {
    "uint8_t": "u8", "uint16_t": "u16", "uint32_t": "u32", "uint64_t": "u64",
    "int8_t": "i8", "int16_t": "i16", "int32_t": "i32", "int64_t": "i64",
    "__uint8_t": "u8", "__uint16_t": "u16", "__uint32_t": "u32", "__uint64_t": "u64",
    "__int8_t": "i8", "__int16_t": "i16", "__int32_t": "i32", "__int64_t": "i64",
    "uintptr_t": "usize", "intptr_t": "isize", "size_t": "usize", "ssize_t": "isize", "ptrdiff_t": "isize",
    "__size_t": "usize", "_ssize_t": "isize", "__uintptr_t": "usize", "__intptr_t": "isize",
    "u8": "u8", "u16": "u16", "u32": "u32", "u64": "u64", "u128": "u128",
    "char16_t": "u16", "char32_t": "u32", "uint_least16_t": "u16", "uint_least32_t": "u32",
    "__builtin_va_list": "va_list", "__gnuc_va_list": "va_list", "va_list": "va_list",
}
PRIM_SIZE = {
    "c_void": (1, 1), "c_char": (1, 1), "c_schar": (1, 1), "c_uchar": (1, 1), "c_short": (2, 2), "c_ushort": (2, 2),
    "c_int": (4, 4), "c_uint": (4, 4), "c_long": (8, 8), "c_ulong": (8, 8), "c_longlong": (8, 8),
    "c_ulonglong": (8, 8), "f32": (4, 4), "f64": (8, 8), "bool": (1, 1), "u128": (16, 16), "i128": (16, 16),
    "u8": (1, 1), "u16": (2, 2), "u32": (4, 4), "u64": (8, 8), "i8": (1, 1), "i16": (2, 2), "i32": (4, 4),
    "i64": (8, 8), "usize": (8, 8), "isize": (8, 8), "va_list": (32, 8),
}

ANON_RE = re.compile(r"(?:(?:struct|union|enum) )?(?:\w+::|\((?:anonymous|unnamed)(?: struct| union| enum)?\)::)*\((?:anonymous|unnamed)(?: struct| union| enum)? at ([^)]+)\)")


def norm_loc(k):
    """Normalize the path part of 'file:L:C'."""
    f, l, c = k.rsplit(":", 2)
    return f"{os.path.normpath(f)}:{l}:{c}"


def tokenize(s):
    s = re.sub(r"__attribute__\(\((?:[^()]|\([^()]*\))*\)\)", "", s)
    s = ANON_RE.sub(lambda m: " __ANON@" + norm_loc(m.group(1).replace(" ", "")) + " ", s)
    return re.findall(r"__ANON@\S+|\.\.\.|[A-Za-z_]\w*|\d+|[*()\[\],]", s)


SPEC_WORDS = {"unsigned", "signed", "short", "long", "int", "char", "float", "double", "void", "_Bool", "__int128"}
QUALS = {"const", "volatile", "restrict", "__restrict", "_Atomic", "__unaligned"}


class P:
    def __init__(self, toks):
        self.t, self.i = toks, 0

    def peek(self, k=0):
        return self.t[self.i + k] if self.i + k < len(self.t) else None

    def take(self):
        if self.i >= len(self.t):
            raise ValueError(f"unexpected end of type: {' '.join(self.t)}")
        self.i += 1
        return self.t[self.i - 1]

    def type_(self):
        const = False
        words = []
        tag = None
        while True:
            p = self.peek()
            if p in QUALS:
                const |= p == "const"
                self.take()
            elif p in ("struct", "union", "enum"):
                tag = self.take()
                words.append(self.take())
            elif p in SPEC_WORDS:
                words.append(self.take())
            elif p and (re.match(r"[A-Za-z_]", p) or p.startswith("__ANON@")) and not words:
                words.append(self.take())
            else:
                break
        name = " ".join(words) if not tag else words[-1]
        base = ("base", name, const, tag)
        return self.declarator(base)

    def declarator(self, base):
        # pointers
        t = base
        while self.peek() == "*":
            self.take()
            c = False
            while self.peek() in QUALS:
                c |= self.take() == "const"
            t = ("ptr", t, c)
        inner = None
        if self.peek() == "(" and self.peek(1) in ("*", "("):
            self.take()
            mark = len(self.t)
            inner_tokens_start = self.i
            # parse inner declarator against a placeholder
            inner = self.declarator(("hole",))
            assert self.take() == ")"
        sufs = []
        while self.peek() in ("[", "("):
            if self.take() == "[":
                n = None
                if self.peek() != "]":
                    n = int(self.take())
                assert self.take() == "]"
                sufs.append(("arr", n))
            else:
                params, variadic = [], False
                while self.peek() != ")":
                    if self.peek() is None:
                        raise ValueError(f"unterminated params: {' '.join(self.t)}")
                    if self.peek() == "...":
                        self.take()
                        variadic = True
                    else:
                        params.append(self.type_())
                    if self.peek() == ",":
                        self.take()
                self.take()
                if len(params) == 1 and params[0] == ("base", "void", False, None):
                    params = []
                sufs.append(("fn", params, variadic))
        for s in reversed(sufs):
            t = ("arr", t, s[1]) if s[0] == "arr" else ("fn", t, s[1], s[2])
        if inner is not None:
            t = fill_hole(inner, t)
        return t


def fill_hole(t, x):
    if t == ("hole",):
        return x
    if t[0] == "ptr":
        return ("ptr", fill_hole(t[1], x), t[2])
    if t[0] == "arr":
        return ("arr", fill_hole(t[1], x), t[2])
    if t[0] == "fn":
        return ("fn", fill_hole(t[1], x), t[2], t[3])
    raise ValueError(t)


def parse_type(s):
    p = P(tokenize(s))
    t = p.type_()
    if p.peek() is not None:
        raise ValueError(f"trailing tokens in {s!r}: {p.t[p.i:]}")
    return t


parse_layouts()

# ---------------------------------------------------------------------------------------------- symbol tables

anon_names = {}      # "file:L:C" -> rust name
tag_names = {}       # ("struct"/"union"/"enum", tag) -> rust name
typedefs = {}        # typedef name -> parsed type (only for real aliases)
fn_typedefs = set()  # typedefs naming function types (rendered as Option<fn>)
enum_types = {}      # rust name -> int type
records = {}         # rust name -> info
known_types = set()  # every rust type name we define
item_module = {}     # rust item name -> module path tuple (for diagnostics)


def anon_key(n):
    f, l, c = loc_of(n)
    return f"{os.path.normpath(f)}:{l}:{c}"


def render(t, ctx="field"):
    """Render a parsed C type as Rust. ctx: field | param | ret"""
    k = t[0]
    if k == "base":
        name, const, tag = t[1], t[2], t[3]
        if name.startswith("__ANON@"):
            key = name[len("__ANON@"):]
            if key not in anon_names:
                warn(f"unknown anonymous type {key}")
                return "c_void"
            return anon_names[key]
        if tag:
            opaque_needed.add((tag, name))  # resolved against our records/newlib at the end
            return tag_names.get((tag, name), name)
        if name in PRIMS:
            r = PRIMS[name]
            return "()" if (r == "c_void" and ctx == "ret") else r
        if name in DIRECT:
            return DIRECT[name]
        if name in fn_typedefs and ctx in ("field", "param", "ret"):
            return name
        used_typedef_names.add(name)
        return name
    if k == "ptr":
        inner = t[1]
        if inner[0] == "fn":
            return render_fn_ptr(inner)
        if inner[0] == "base" and inner[1] in fn_typedefs:
            return inner[1]  # pointer to function typedef == our Option<fn> alias
        c = inner[0] == "base" and inner[2]
        if inner[0] == "ptr" and False:
            pass
        r = render(inner, "field")
        if r == "()":
            r = "c_void"
        return ("*const " if c else "*mut ") + r
    if k == "arr":
        if ctx == "param":
            return "*mut " + render(t[1], "field")
        n = t[2] if t[2] is not None else 0
        return f"[{render(t[1], 'field')}; {n}]"
    if k == "fn":
        return render_fn_ptr(t)
    raise ValueError(t)


def render_fn_ptr(fn):
    args = ", ".join(render(p, "param") for p in fn[2])
    if fn[3]:
        args = (args + ", ..." if args else "...")
    ret = render(fn[1], "ret")
    tail = "" if ret == "()" else f" -> {ret}"
    return f"Option<unsafe extern \"C\" fn({args}){tail}>"


def size_align(t):
    """(size, align) on aarch64 for a parsed C type."""
    k = t[0]
    if k == "ptr" or k == "fn":
        return 8, 8
    if k == "arr":
        s, a = size_align(t[1])
        return s * (t[2] or 0), a
    name, tag = t[1], t[3]
    if name.startswith("__ANON@"):
        lay = layouts.get("anon:" + name[len("__ANON@"):])
        return lay["size"], lay["align"]
    if tag == "enum" or (not tag and name in enum_typedef_c):
        nm = tag_names.get(("enum", name), name)
        return (8, 8) if enum_types.get(nm, "u32") in ("u64", "i64") else (4, 4)
    if tag:
        lay = layouts.get(f"{tag}:{name}")
        if lay is None:
            raise KeyError(f"no layout for {tag} {name}")
        return lay["size"], lay["align"]
    if name in PRIMS:
        return PRIM_SIZE[PRIMS[name]]
    if name in DIRECT:
        return PRIM_SIZE[DIRECT[name]]
    if name in fn_typedefs:
        return 8, 8
    if name in typedef_c:
        return size_align(typedef_c[name])
    raise KeyError(f"unknown type {name}")


typedef_c = {}        # typedef name -> parsed C type (all typedefs, incl. newlib)
enum_typedef_c = set()
opaque_needed = set()
used_typedef_names = set()

# ---------------------------------------------------------------------------------------------- doc comments

_src_cache = {}


def src_lines(path):
    if path not in _src_cache:
        _src_cache[path] = open(path, encoding="utf-8", errors="replace").read().splitlines()
    return _src_cache[path]


def conv_inline(s):
    s = re.sub(r"[\\@]ref\s+(\w+(?:\(\))?)", r"`\1`", s)
    s = re.sub(r"[\\@][pca]\s+(\w+)", r"`\1`", s)
    s = re.sub(r'<a href="([^"]+)">([^<]+)</a>', r"[\2](\1)", s)
    s = re.sub(r"<(?!https?:)", "&lt;", s)
    # bare URLs -> autolinks (skip ones already inside a markdown link or <...>)
    s = re.sub(r"(?<![(<])(https?://[^\s)>]+[^\s)>.,;:])", r"<\1>", s)
    # [word] would be parsed as an intra-doc link
    s = re.sub(r"\[([A-Za-z_0-9]+)\](?!\()", r"\\[\1\\]", s)
    return s


def conv_doc(lines):
    brief, params, extra = [], [], []
    cur = brief
    in_code = False
    for raw in lines:
        raw = raw.replace("\t", "    ")
        l = raw.strip()
        if l in ("/**", "*/", "/*!"):
            continue
        l = re.sub(r"^/\*\*<?|^/\*!<?|^///<?|^//!<?|\*/$", "", l).strip()
        l = l.lstrip("*").strip() if not in_code else re.sub(r"^\*\s?", "", raw.strip())
        if re.match(r"[@\\](code|verbatim)\b", l):
            cur.extend(["", "```text"])
            in_code = True
            continue
        if re.match(r"[@\\](endcode|endverbatim)\b", l):
            cur.append("```")
            in_code = False
            continue
        if in_code:
            cur.append(l)
            continue
        if not l or re.match(r"[@\\](\{|\}|name\b|file\b|author\b|copyright\b|defgroup\b|ingroup\b|addtogroup\b)", l):
            if not l and cur is not params and cur and cur[-1] != "":
                cur.append("")
            continue
        if re.match(r"[@\\]brief\b", l):
            cur = brief
            brief.append(conv_inline(l[6:].strip()))
        elif re.match(r"[@\\]t?param\b", l):
            m = re.match(r"[@\\]t?param(?:\[([\w, ]+)\])?\s+(\w+)\s*(.*)", l)
            if not m:
                continue
            tag = f" ({m.group(1).replace(' ', '')})" if m.group(1) else ""
            cur = params
            params.append(f"- `{m.group(2)}`{tag}: {conv_inline(m.group(3))}")
        elif re.match(r"[@\\]returns?\b", l):
            cur = extra
            extra += ["", f"**Returns:** {conv_inline(re.sub(r'^[@\\\\]returns?', '', l).strip())}"]
        elif re.match(r"[@\\]\w+", l):
            m = re.match(r"[@\\](\w+)\s*(.*)", l)
            cur = extra
            extra += ["", f"**{m.group(1).capitalize()}:** {conv_inline(m.group(2))}"]
        else:
            cur.append(("  " if cur is params else "") + conv_inline(l))
    out = list(brief)
    if params:
        out += ["", "# Arguments"] + params
    out += extra
    while out and out[0] == "":
        out.pop(0)
    while out and out[-1] == "":
        out.pop()
    # collapse blank runs
    res = []
    for l in out:
        if l == "" and res and res[-1] == "":
            continue
        res.append(l)
    return res


def doc_for(n):
    """Doc comment for a declaration, read from the source around its location."""
    f, dl, _ = loc_of(n)
    if not f or not dl:
        return []
    lines = src_lines(f)
    line = lines[dl - 1]
    m = re.search(r"(///<|//!<|/\*\*<|/\*!<)(.*?)(\*/)?\s*$", line)
    if m:
        return conv_doc([m.group(2)])
    j = dl - 2
    # skip continuation lines of a multi-line declaration (e.g. return type on its own line)
    if n.get("kind") == "FunctionDecl":
        while j >= 0 and lines[j].strip() and not re.search(r"\*/\s*$|^\s*///|;\s*(//.*)?$|\{\s*$|\}\s*$", lines[j]):
            j -= 1
    if j < 0:
        return []
    prev = lines[j].strip()
    block = []
    if prev.endswith("*/"):
        while j >= 0:
            block.insert(0, lines[j])
            if re.search(r"/\*\*|/\*!", lines[j]):
                return conv_doc(block)
            if "/*" in lines[j]:
                return []
            j -= 1
        return []
    while j >= 0 and re.match(r"\s*///(?!<)|\s*//!(?!<)", lines[j]):
        block.insert(0, lines[j])
        j -= 1
    return conv_doc(block)


def emit_doc(out, doc, indent=""):
    for d in doc:
        out.append(f"{indent}/// {d}".rstrip())


# ---------------------------------------------------------------------------------------------- collect decls

def file_decls():
    """Top-level decls grouped by header (in source order), plus newlib decls by name."""
    per = defaultdict(list)
    ext = {}
    for d in ast["inner"]:
        f, _, _ = loc_of(d)
        if d.get("isImplicit"):
            continue
        rel = rel_of(f)
        if rel:
            per[rel].append(d)
        else:
            nm = d.get("name")
            if nm:
                if d["kind"] == "RecordDecl":
                    if d.get("completeDefinition") or ("rec", nm) not in ext:
                        ext[("rec", nm)] = d
                elif d["kind"] == "TypedefDecl":
                    ext[("td", nm)] = d
                elif d["kind"] == "EnumDecl":
                    ext[("enum", nm)] = d
    return per, ext


per_file, ext_decls = file_decls()

# First pass over ALL typedefs to know their C types (needed for size/align and fn typedef detection).
for d in ast["inner"]:
    if d["kind"] == "TypedefDecl" and "name" in d:
        try:
            typedef_c[d["name"]] = parse_type(d["type"]["qualType"])
        except Exception as e:  # builtins like __NSConstantString
            pass

# ---------------------------------------------------------------------------------------------- naming pass

def name_records(nodes, rel, parent=None):
    """Assign Rust names to records/enums (including anonymous and nested ones)."""
    pending_anon = []
    for i, d in enumerate(nodes):
        k = d["kind"]
        if k in ("RecordDecl", "EnumDecl"):
            tag = d.get("tagUsed", "enum")
            nm = d.get("name")
            if nm:
                tag_names[(tag, nm)] = nm
            else:
                key = anon_key(d)
                # typedef'd anonymous? look ahead for TypedefDecl owning it
                tname = None
                for nx in nodes[i + 1:i + 3]:
                    if nx["kind"] == "TypedefDecl":
                        for c in nx.get("inner", []):
                            if c.get("isTagOwned") and c.get("decl", {}).get("name", None) == "" and \
                                    c.get("decl", {}).get("id") == d.get("id"):
                                tname = nx["name"]
                if tname is None:
                    # match by id regardless of distance
                    for nx in nodes[i + 1:]:
                        if nx["kind"] == "TypedefDecl" and any(
                                c.get("decl", {}).get("id") == d.get("id") for c in nx.get("inner", [])):
                            tname = nx["name"]
                            break
                if tname:
                    anon_names[key] = tname
                    tag_names[(tag, tname)] = tname
                    typedef_anon.append((tag, tname, key))
                elif parent:
                    # nested anonymous: named after the field that uses it (resolved below)
                    pending_anon.append((key, d))
                else:
                    anon_names[key] = None  # anonymous top-level (e.g. anonymous enum of constants)
    if parent:
        # name nested anonymous records after the field that has their type
        anon_idx = 0
        for key, d in pending_anon:
            fname = None
            for f in nodes:
                if f["kind"] == "FieldDecl" and key in {norm_loc(x.replace(" ", "")) for x in ANON_RE.findall(f["type"]["qualType"])}:
                    fname = f.get("name")
                    break
            if not fname:
                fname = f"anon{anon_idx}"
                anon_idx += 1
                d["_anon_field"] = fname
            anon_names[key] = f"{parent}_{fname}"
    for d in nodes:
        if d["kind"] == "RecordDecl":
            own = d.get("name") or anon_names.get(anon_key(d))
            if own is None:
                raise RuntimeError(f"unnamed record at {anon_key(d)}")
            name_records(d.get("inner", []), rel, parent=own)


typedef_anon = []
for rel, nodes in per_file.items():
    name_records(nodes, rel)
for tag, tname, key in typedef_anon:
    lay = layouts.get("anon:" + key.replace(" ", ""))
    if lay is not None:
        layouts.setdefault(f"{tag}:{tname}", lay)

# enums: integer types
CAST_BITS = {"int": (32, True), "unsigned int": (32, False), "long": (64, True), "unsigned long": (64, False),
             "long long": (64, True), "unsigned long long": (64, False)}


def enum_init_value(c):
    """Value of an enumerator's initializer: the innermost ConstantExpr, converted through any
    enclosing implicit casts exactly as C does."""
    casts = []
    n = c
    while True:
        kids = [x for x in n.get("inner", []) if isinstance(x, dict) and "kind" in x]
        if not kids:
            return None
        n = kids[0]
        if n["kind"] == "ConstantExpr" and "value" in n:
            v = int(n["value"])
            break
        if n["kind"] in ("ImplicitCastExpr", "CStyleCastExpr"):
            casts.append(n["type"]["qualType"])
    for ty in reversed(casts):
        bits_signed = CAST_BITS.get(ty)
        if bits_signed:
            bits, signed = bits_signed
            v &= (1 << bits) - 1
            if signed and v >= 1 << (bits - 1):
                v -= 1 << bits
    return v


def enum_constants(d):
    vals, last = [], -1
    for c in d.get("inner", []):
        if c["kind"] != "EnumConstantDecl":
            continue
        v = enum_init_value(c)
        if v is None:
            v = last + 1
        last = v
        vals.append((c, v))
    return vals


def enum_int_type(vals):
    if not vals:
        return "u32"
    lo = min(v for _, v in vals)
    hi = max(v for _, v in vals)
    if lo < 0:
        return "i32" if lo >= -2**31 and hi < 2**31 else "i64"
    return "u32" if hi < 2**32 else "u64"


all_enum_nodes = []


def collect_enums(nodes):
    for d in nodes:
        if d["kind"] == "EnumDecl":
            all_enum_nodes.append(d)
        if d["kind"] == "RecordDecl":
            collect_enums(d.get("inner", []))


for nodes in per_file.values():
    collect_enums(nodes)
for d in all_enum_nodes:
    nm = d.get("name") or anon_names.get(anon_key(d))
    if nm:
        enum_types[nm] = d.get("fixedUnderlyingType", {}).get("qualType") and \
            DIRECT.get(d["fixedUnderlyingType"]["qualType"], "u32") or enum_int_type(enum_constants(d))
for name, t in typedef_c.items():
    if t[0] == "base" and t[3] == "enum":
        enum_typedef_c.add(name)
    if t[0] == "fn":
        fn_typedefs.add(name)

# ---------------------------------------------------------------------------------------------- union containment

rec_nodes = {}


def index_records(nodes):
    for d in nodes:
        if d["kind"] == "RecordDecl" and d.get("completeDefinition"):
            tag = d.get("tagUsed", "struct")
            if d.get("name"):
                rec_nodes[f"{tag}:{d['name']}"] = d
            else:
                rec_nodes["anon:" + anon_key(d)] = d
                tn = anon_names.get(anon_key(d))
                if tn:
                    rec_nodes.setdefault(f"{tag}:{tn}", d)
            index_records(d.get("inner", []))


index_records(ast["inner"])
_union_memo = {}


def type_has_union(t):
    if t[0] == "arr":
        return type_has_union(t[1])
    if t[0] != "base":
        return False
    name, tag = t[1], t[3]
    if name.startswith("__ANON@"):
        return record_has_union("anon:" + name[len("__ANON@"):])
    if tag == "union":
        return True
    if tag == "struct":
        return record_has_union(f"struct:{name}")
    if name in typedef_c and name not in DIRECT:
        k = "td:" + name
        if k not in _union_memo:
            _union_memo[k] = False  # cycle guard
            _union_memo[k] = type_has_union(typedef_c[name])
        return _union_memo[k]
    return False


def record_has_union(key):
    if key in _union_memo:
        return _union_memo[key]
    _union_memo[key] = False
    d = rec_nodes.get(key)
    r = False
    if d is not None:
        r = d.get("tagUsed") == "union" or any(
            type_has_union(parse_type(f["type"]["qualType"])) for f in d.get("inner", []) if f["kind"] == "FieldDecl")
    _union_memo[key] = r
    return r


# ---------------------------------------------------------------------------------------------- emitters

inline_ports = {}  # module path -> {cname: rust source}


def load_inline_ports(modpath):
    p = os.path.join(CRATE, "tools", "inline_ports", *modpath) + ".rs"
    if not os.path.exists(p):
        return {}
    src = open(p).read()
    out = {}
    for m in re.finditer(r"^// inline: (\w+)\n(.*?)(?=^// inline: |\Z)", src, re.S | re.M):
        out[m.group(1)] = m.group(2).rstrip() + "\n"
    return out


def bf_storage(bits):
    for t, n in (("u8", 8), ("u16", 16), ("u32", 32), ("u64", 64)):
        if bits <= n:
            return t, n
    raise ValueError(bits)


def emit_record(d, name, out, modpath):
    tag = d.get("tagUsed", "struct")
    key = f"{tag}:{d['name']}" if d.get("name") else "anon:" + anon_key(d).replace(" ", "")
    lay = layouts.get(key) or layouts.get(f"{tag}:{name}")
    if not d.get("completeDefinition") or lay is None:
        if d.get("name") and f"{tag}:{d['name']}" in layouts:
            return  # forward declaration of a record defined elsewhere
        if name not in emitted_types:
            emit_doc(out, doc_for(d))
            out.append("#[repr(C)]")
            out.append("#[derive(Debug)]")
            out.append(f"pub struct {name} {{\n    _opaque: [u8; 0],\n}}\n")
            emitted_types.add(name)
            known_types.add(name)
        return
    if name in emitted_types:
        return
    emitted_types.add(name)
    known_types.add(name)
    inner = d.get("inner", [])
    # nested records/enums first
    nested_out = []
    for c in inner:
        if c["kind"] == "RecordDecl":
            cn = c.get("name") or anon_names.get(anon_key(c))
            emit_record(c, cn, nested_out, modpath)
        elif c["kind"] == "EnumDecl":
            emit_enum(c, nested_out)
    fields = [c for c in inner if c["kind"] == "FieldDecl"]
    lf = {f[0]: f for f in lay["fields"]}
    packed = any(c["kind"] == "PackedAttr" for c in inner)
    explicit_align = None
    for c in inner:
        if c["kind"] == "AlignedAttr":
            explicit_align = lay["align"]
    is_union = tag == "union"
    has_union_inside = lay["has_union"] or record_has_union(key) or record_has_union(f"{tag}:{name}")

    rows = []     # (doc, rust_name, rust_type, byte_off, size, align)
    bitgroups = []
    cur_bits = None
    anon_i = 0
    if len(fields) != len(lay["fields"]):
        raise RuntimeError(f"{name}: {len(fields)} fields vs {len(lay['fields'])} layout entries")
    for fi, f in enumerate(fields):
        fname = f.get("name")
        if not fname:
            # implicit field for an anonymous member
            m = ANON_RE.search(f["type"]["qualType"])
            k = norm_loc(m.group(1).replace(" ", "")) if m else None
            nested = next((c for c in inner if c["kind"] == "RecordDecl" and anon_key(c).replace(" ", "") == k), None)
            fname = nested.get("_anon_field") if nested is not None else None
            if not fname:
                fname = f"anon{anon_i}"
                anon_i += 1
            layname = "(anonymous)"
        else:
            layname = fname
        ct = parse_type(f["type"]["qualType"])
        entry = lay["fields"][fi]
        doc = doc_for(f)
        if f.get("isBitfield"):
            width = entry[2]
            bitoff = entry[1]
            if width == 0 or not f.get("name"):
                continue
            end_bit = bitoff + width
            if cur_bits is not None and (end_bit + 7) // 8 - cur_bits["start"] // 8 > 16:
                cur_bits = None  # keep each storage run within 128 bits
            if cur_bits is None:
                start = (bitoff // 8) * 8
                cur_bits = {"name": f"_bitfield_{len(bitgroups)}", "start": start, "end": end_bit, "fields": []}
                bitgroups.append(cur_bits)
                rows.append([[], cur_bits["name"], None, start // 8, None, 1, "bitfield"])
                cur_row = rows[-1]
            cur_bits["end"] = max(cur_bits["end"], end_bit)
            nbytes = (cur_bits["end"] + 7) // 8 - cur_bits["start"] // 8
            cur_row[2] = f"[u8; {nbytes}]"
            cur_row[4] = nbytes
            vt = render(ct, "field")
            cur_bits["fields"].append((doc, ident(fname), bitoff - cur_bits["start"], width, vt))
            bitfield_checks.append((name, ident(fname), bitoff, width, vt, lay["size"]))
        else:
            cur_bits = None
            rt = render(ct, "field")
            if "union" in f["type"]["qualType"] or set(re.findall(r"\w+", rt)) & union_types:
                has_union_inside = True
            sz, al = size_align(ct)
            rows.append([doc, ident(fname), rt, entry[1] // 8, sz, al, "field"])

    # Simulate Rust's repr(C) layout and insert padding where clang differs. Prefer a plain
    # (unpacked) layout; fall back to packed(N) only when clang's offsets require it.
    out.extend(nested_out)
    emit_doc(out, doc_for(d))

    def simulate(pack):
        body, problems = [], []
        natural_align, pos, pad_i = 1, 0, 0
        for doc, rn, rt, off, sz, al, kind in rows:
            eff_al = min(al, pack) if pack else al
            off_nat = 0 if is_union else (pos + eff_al - 1) // eff_al * eff_al
            if not is_union and off > off_nat:
                body.append(f"    pub _pad{pad_i}: [u8; {off - off_nat}],")
                pad_i += 1
                off_nat = off
            if not is_union and off != off_nat:
                problems.append(f"{name}.{rn}: clang offset {off} < rust {off_nat}")
            natural_align = max(natural_align, eff_al)
            emit_doc(body, doc, "    ")
            body.append(f"    pub {rn}: {rt},")
            pos = max(pos, off_nat + sz) if not is_union else max(pos, sz)
        repr_ = ["C"]
        if pack:
            repr_.append("packed" if pack == 1 else f"packed({pack})")
        if lay["align"] > natural_align:
            if pack:
                problems.append(f"{name}: needs packed and align({lay['align']})")
            repr_.append(f"align({lay['align']})")
            natural_align = lay["align"]
        elif lay["align"] < natural_align:
            problems.append(f"{name}: rust align {natural_align} > clang align {lay['align']}")
        end = (pos + natural_align - 1) // natural_align * natural_align
        if end < lay["size"]:
            body.append(f"    pub _pad{pad_i}: [u8; {lay['size'] - end}],")
        elif end > lay["size"]:
            problems.append(f"{name}: rust size {end} > clang size {lay['size']}")
        return body, repr_, problems

    body, repr_, problems = simulate(0)
    if problems:
        body, repr_, problems = simulate(lay["align"])
        for pr in problems:
            warn(pr)
    out.append(f"#[repr({', '.join(repr_)})]")
    if is_union:
        union_types.add(name)
        out.append("#[derive(Clone, Copy)]")
        out.append(f"pub union {name} {{")
    else:
        if has_union_inside:
            union_types.add(name)
            out.append("#[derive(Clone, Copy)]")
        else:
            out.append("#[derive(Debug, Clone, Copy)]")
        out.append(f"pub struct {name} {{")
    out.extend(body)
    out.append("}")
    out.append("")
    zeroed_defaults.append(name)
    if bitgroups:
        out.append(f"impl {name} {{")
        for g in bitgroups:
            out.append(f"    bitfields!({g['name']} {{")
            for doc, fn, shift, width, vt in g["fields"]:
                emit_doc(out, doc, "        ")
                out.append(f"        {fn} / set_{fn}: {shift}, {width} => {vt};")
            out.append("    });")
        out.append("}")
        out.append("")
    record_layout_checks.append((name, key, rows, lay, "::".join(modpath)))


lf_used = set()
bitfield_checks = []
union_types = set()
zeroed_defaults = []
emitted_types = set()
record_layout_checks = []
emitted_consts = set()


def fmt_int(v, src=None):
    if src:
        return src
    return str(v) if -4096 < v < 4096 else (hex(v) if v >= 0 else "-" + hex(-v))


def enum_src_expr(c, ctype, consts_so_far):
    """Keep simple source expressions (literals, BIT(n), refs to earlier constants) for readability."""
    f, l, _ = loc_of(c)
    if not f:
        return None
    line = src_lines(f)[l - 1]
    m = re.search(r"\b" + re.escape(c["name"]) + r"\s*=\s*(.+?)\s*(,|$|///|//|/\*)", line)
    if not m:
        return None
    e = m.group(1).strip().rstrip(",").strip()
    e = re.sub(r"\b(0x[0-9a-fA-F]+|\d+)[uUlL]+\b", r"\1", e)
    toks = re.findall(r"[A-Za-z_]\w*|0x[0-9a-fA-F]+|\d+|<<|>>|[|&()~+\-*]", e)
    if "".join(toks) != re.sub(r"\s+", "", e):
        return None
    for t in toks:
        if re.match(r"[A-Za-z_]", t) and t not in consts_so_far and t not in ("BIT", "BITL"):
            return None
    if "BITL" in toks and ctype not in ("u64", "i64"):
        return None
    if "BIT" in toks and ctype in ("u64", "i64", "i32"):
        return None
    if "~" in toks and ctype.startswith("i"):
        return None
    if re.search(r"(^|[^\w])-", e) and ctype.startswith("u"):
        return None
    return e


def emit_enum(d, out):
    vals = enum_constants(d)
    nm = d.get("name") or anon_names.get(anon_key(d))
    ty = enum_types.get(nm) if nm else enum_int_type(vals)
    doc = doc_for(d)
    if nm:
        if nm in emitted_types:
            return
        emitted_types.add(nm)
        known_types.add(nm)
        emit_doc(out, doc)
        out.append(f"pub type {nm} = {ty};")
        cty = nm
    else:
        emit_doc(out, doc)
        cty = ty
    seen = set()
    for c, v in vals:
        if c["name"] in emitted_consts:
            continue
        emitted_consts.add(c["name"])
        emit_doc(out, doc_for(c))
        src = enum_src_expr(c, ty, seen)
        out.append(f"pub const {c['name']}: {cty} = {fmt_int(v, src)};")
        const_checks.append((c["name"], v))
        seen.add(c["name"])
    out.append("")


const_checks = []
fn_checks = []


def fn_sig(d):
    ft = parse_type(re.sub(r"__attribute__\(\((?:[^()]|\([^()]*\))*\)\)", "", d["type"]["qualType"]))
    params = [c for c in d.get("inner", []) if c["kind"] == "ParmVarDecl"]
    args = []
    for i, p in enumerate(params):
        pn = ident(p.get("name") or f"arg{i}")
        args.append(f"{pn}: {render(parse_type(p['type']['qualType']), 'param')}")
    if d.get("variadic"):
        args.append("...")
    noret = "noreturn" in d["type"]["qualType"] or any(c["kind"] in ("NoReturnAttr", "C11NoReturnAttr") for c in d.get("inner", []))
    ret = "!" if noret else render(ft[1], "ret")
    tail = "" if ret == "()" else f" -> {ret}"
    return f"{ident(d['name'])}({', '.join(args)}){tail}"


def emit_module(rel, nodes):
    modpath = header_to_module(rel)
    ports = load_inline_ports(modpath)
    used_ports = set()
    z0 = len(zeroed_defaults)
    out = []
    externs = []
    fn_seen = set()
    for i, d in enumerate(nodes):
        k = d["kind"]
        if k == "RecordDecl":
            nm = d.get("name") or anon_names.get(anon_key(d))
            if nm is None:
                continue
            # skip forward declarations when a definition exists in this file or elsewhere
            if not d.get("completeDefinition"):
                tagk = f"{d.get('tagUsed')}:{d.get('name')}"
                if tagk in layouts:
                    continue
            emit_record(d, nm, out, modpath)
        elif k == "EnumDecl":
            emit_enum(d, out)
        elif k == "TypedefDecl":
            tn = d["name"]
            t = parse_type(d["type"]["qualType"])
            if tn in DIRECT:
                continue
            if t[0] == "base" and (t[3] or t[1].startswith("__ANON@")):
                target = render(t)
                if target == tn:
                    continue
                emit_doc(out, doc_for(d))
                out.append(f"pub type {tn} = {target};\n")
                known_types.add(tn)
                continue
            emit_doc(out, doc_for(d))
            if t[0] == "fn":
                out.append(f"pub type {tn} = {render_fn_ptr(t)};\n")
            else:
                out.append(f"pub type {tn} = {render(t)};\n")
            known_types.add(tn)
        elif k == "FunctionDecl":
            nm = d["name"]
            is_static = d.get("storageClass") == "static"
            if is_static:
                if nm in ports:
                    emit_doc(out, doc_for(d))
                    out.append(ports[nm])
                    used_ports.add(nm)
                else:
                    missing_ports.append((rel, nm))
                continue
            if nm in fn_seen:
                continue
            fn_seen.add(nm)
            doc = doc_for(d)
            emit_doc(externs, doc, "    ")
            dep = next((c for c in d.get("inner", []) if c["kind"] == "DeprecatedAttr"), None)
            if dep is not None:
                msg = dep.get("message")
                externs.append(f"    #[deprecated{'(note = ' + json.dumps(msg) + ')' if msg else ''}]")
            externs.append(f"    pub fn {fn_sig(d)};")
            externs.append("")
            fn_checks.append(nm)
        elif k == "VarDecl":
            emit_doc(externs, doc_for(d), "    ")
            externs.append(f"    pub static mut {ident(d['name'])}: {render(parse_type(d['type']['qualType']))};")
            externs.append("")
    if zeroed_defaults[z0:]:
        out.append("impl_zeroed_default!(" + ", ".join(zeroed_defaults[z0:]) + ");")
        out.append("")
    for nm in ports:
        if nm not in used_ports:
            # helpers that replace macros (no C declaration)
            out.append(ports[nm])
    if externs:
        while externs[-1] == "":
            externs.pop()
        out.append('unsafe extern "C" {')
        out.extend(externs)
        out.append("}")
        out.append("")
    return modpath, out


missing_ports = []
modules = OrderedDict()

for rel in sorted(per_file):
    if rel in HANDWRITTEN or rel in SKIP_HEADERS:
        # still register names defined there
        for d in per_file[rel]:
            if d["kind"] in ("RecordDecl", "EnumDecl", "TypedefDecl") and d.get("name"):
                known_types.add(d["name"])
                emitted_types.add(d["name"])
            if d["kind"] == "EnumDecl":
                for c, _ in enum_constants(d):
                    emitted_consts.add(c["name"])
        continue

macros = json.load(open(os.path.join(SCRATCH, "macros.json")))  # rel -> [(name, rust_type, value, doc)]

for rel in sorted(set(per_file) | set(macros)):
    if rel in HANDWRITTEN or rel in SKIP_HEADERS:
        continue
    try:
        modpath, body = emit_module(rel, per_file.get(rel, []))
    except Exception as e:
        import traceback
        traceback.print_exc()
        print("FAILED in", rel, file=sys.stderr)
        raise
    modules[tuple(modpath)] = (rel, body)

# ---------------------------------------------------------------------------------------------- newlib types

def emit_newlib():
    out = []
    pending = sorted((used_typedef_names - known_types - set(DIRECT) - set(PRIMS)) | {n for _, n in opaque_needed})
    done = set()
    while pending:
        n = pending.pop(0)
        if n in done or n in known_types:
            continue
        done.add(n)
        if ("td", n) in ext_decls:
            d = ext_decls[("td", n)]
            t = parse_type(d["type"]["qualType"])
            before = set(used_typedef_names) | {x for _, x in opaque_needed}
            r = render_fn_ptr(t) if t[0] == "fn" else render(t)
            if r != n:
                out.append(f"pub type {n} = {r};")
            known_types.add(n)
        elif ("rec", n) in ext_decls:
            d = ext_decls[("rec", n)]
            before = set(used_typedef_names) | {x for _, x in opaque_needed}
            emit_record(d, n, out, ["newlib"])
        else:
            warn(f"unresolved external type {n}")
            continue
        new = (set(used_typedef_names) | {x for _, x in opaque_needed}) - known_types - done - set(DIRECT) - set(PRIMS)
        pending.extend(sorted(new))
    return out


z0 = len(zeroed_defaults)
newlib_body = emit_newlib()
if zeroed_defaults[z0:]:
    newlib_body += ["", "impl_zeroed_default!(" + ", ".join(zeroed_defaults[z0:]) + ");"]

# ---------------------------------------------------------------------------------------------- write files

HEADER = """#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;
"""


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write(text)


def module_consts(rel):
    out = []
    for name, ty, val, doc in macros.get(rel, []):
        if name in emitted_consts or name in known_types:
            continue
        emitted_consts.add(name)
        emit_doc(out, doc)
        out.append(f"pub const {name}: {ty} = {val};")
    if out:
        out.append("")
    return out


all_mods = {}
for modpath, (rel, body) in modules.items():
    consts = module_consts(rel)
    text = f"//! `{rel}`\n\n" + HEADER + "\n" + "\n".join(consts + body).rstrip() + "\n"
    text = re.sub(r"\n{3,}", "\n\n", text)
    write(os.path.join(OUT, *modpath[:-1], modpath[-1] + ".rs"), text)
    all_mods[modpath] = rel

nl = "//! Types from devkitA64's newlib that the libnx headers refer to.\n\n" + HEADER.replace(
    "#[allow(unused_imports)]\nuse crate::newlib::*;\n", "") + "\n" + \
    "/// `va_list` is a 32-byte struct on AArch64 that is passed by reference; Rust has no stable equivalent.\npub type va_list = *mut c_void;\n\n" + \
    "\n".join(newlib_body) + "\n"
write(os.path.join(OUT, "newlib.rs"), re.sub(r"\n{3,}", "\n\n", nl))

def write_mod_tree():
    root_mods = []
    for dirpath, dirnames, filenames in os.walk(OUT):
        rel = os.path.relpath(dirpath, OUT)
        children = sorted({f[:-3] for f in filenames if f.endswith(".rs") and f not in ("mod.rs", "lib.rs", "main.rs")}
                          | set(dirnames))
        if rel == "tests" or rel.startswith("tests/"):
            continue  # hand-written test module
        if rel == ".":
            root_mods = children
            continue
        hdr = f"//! `include/{'switch/' if not rel.startswith('bsd') else ''}{rel.replace('bsd/', '', 1) if rel != 'bsd' else ''}`\n\n"
        body = "#![allow(clippy::module_inception)]\n\n" + "".join(f"pub mod {c};\n" for c in children) + "\n" + "".join(f"pub use {c}::*;\n" for c in children)
        write(os.path.join(dirpath, "mod.rs"), hdr + body)
    root_mods = [m for m in root_mods if m != "tests"]
    switch_mods = [m for m in root_mods if m not in ("bsd", "newlib")]
    lib = LIB_HEADER + "".join(f"pub mod {m};\n" for m in root_mods) + "\n" + \
        "".join(f"pub use {m}::*;\n" for m in switch_mods) + \
        "\n// Lets the tests use `libnx::...` paths like an external user would.\n#[cfg(test)]\n" \
        "extern crate self as libnx;\n#[cfg(test)]\nmod tests;\n"
    write(os.path.join(OUT, "lib.rs"), lib)


LIB_HEADER = """//! Rust port of the libnx headers (`include/switch.h` plus the BSD socket headers).
//!
//! Module layout mirrors `include/switch/`, and everything from it is re-exported at the crate root
//! like the C umbrella header. The BSD socket headers (`<sys/socket.h>`, `<netinet/in.h>`, ...), which
//! `switch.h` does not include either, live under [`bsd`]; the devkitA64 newlib types they need are
//! in [`newlib`]. Names are kept identical to libnx so the C documentation applies.
//! `extern` declarations link against `libnx.a`; `static inline` helpers are ported to Rust.
//!
//! Most of this crate is generated from the C headers by clang; see `tools/`.

#![cfg_attr(not(test), no_std)]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

"""

write_mod_tree()

# record what was generated for the other scripts
json.dump({
    "modules": {"/".join(k): v for k, v in all_mods.items()},
    "records": [(n, k, [(r[1], r[3], r[6]) for r in rows], lay["size"], lay["align"], m) for n, k, rows, lay, m in record_layout_checks],
    "zeroed": zeroed_defaults,
    "enum_consts": const_checks,
    "functions": fn_checks,
    "missing_ports": missing_ports,
    "bitfields": bitfield_checks,
    "unions": sorted(union_types),
}, open(os.path.join(SCRATCH, "gen_out.json"), "w"), indent=0)

print(f"modules: {len(all_mods)}  records: {len(record_layout_checks)}  fns: {len(fn_checks)}  "
      f"enum consts: {len(const_checks)}  missing inline ports: {len(missing_ports)}")
for w in warnings:
    print("WARN", w)
