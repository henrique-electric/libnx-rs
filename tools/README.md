# Binding generator

Most of `src/` is generated from the C headers in `include/` (libnx plus its BSD socket headers).
Run `tools/regen.sh` after changing the headers or the generator. It needs `clang`, `python3`
and `git`, and works in `target/libnx-gen/`.

What it does:

1. Fetches devkitA64's newlib headers (sparse git clone of `devkitPro/newlib`, headers only) so
   clang sees the same `time_t`, `_LOCK_T`, `struct timespec`, ... as a real Switch build.
2. Has clang (`tools/cc.sh`, targeting `aarch64-none-elf`) dump the JSON AST, the record layouts,
   and the macro definitions of a translation unit that includes every header.
3. `macros.py` evaluates every object-like `#define` by compiling it for aarch64 and reading the
   values back from LLVM IR (non-constant macros are dropped automatically).
4. `gen.py` writes one Rust module per header, plus `src/newlib.rs` (newlib types the headers use),
   the `mod.rs` files and `src/lib.rs`. Struct layouts come from clang's own aarch64 layout dump:
   explicit padding / `align` / `packed` are inserted wherever Rust's `repr(C)` would differ, and
   bitfields are stored as the exact bytes they occupy (accessors via the `bitfields!` macro).
5. `gen_tests.py` writes `src/tests/generated_layout.rs`, which checks at compile time that every
   struct's size, alignment and field offsets, and every constant, match clang's aarch64 values,
   and checks every bitfield's bit position at runtime.

## Hand-written parts

- `inline_ports/<module>.rs`: Rust ports of the headers' `static inline` functions and of the
  useful function-like macros. Each item starts with a `// inline: <name>` marker. A port named
  after a C inline function replaces it in place (with the C doc comment); other ports are
  appended to the module. `gen.py` fails to list any inline function that has no port.
- `src/types.rs`, `src/result.rs`, `src/arm/{tls,counter,cache,thread_context}.rs`,
  `src/kernel/svc.rs`, `src/sf/{hipc,cmif,service,tipc}.rs`, `src/services/sm.rs` were ported by
  hand and are skipped by the generator (see `HANDWRITTEN` in `gen.py`).

Not ported: macros that only make sense inside the BSD kernel (`BPF_TAP`, `ROTATE_BUFFERS`,
`RT_GEN`, ...), C-only type tricks (`serviceMacroDetectIsPointer`), and stringizing macros
(`CONSOLE_ESC`; its uses are available as constants).
