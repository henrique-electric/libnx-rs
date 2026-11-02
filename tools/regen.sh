#!/bin/bash
# Regenerates the Rust bindings from include/ (see tools/README.md).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORK="$ROOT/target/libnx-gen"
mkdir -p "$WORK"

# 1. devkitA64's newlib headers (only the headers are needed; nothing is installed).
if [ ! -d "$WORK/sysroot/include" ]; then
    rm -rf "$WORK/newlib"
    git clone -q --depth 1 --filter=blob:none --sparse -b devkitPro https://github.com/devkitPro/newlib.git "$WORK/newlib"
    git -C "$WORK/newlib" sparse-checkout set newlib/libc/include newlib/libc/machine/aarch64
    mkdir -p "$WORK/sysroot/include"
    cp -r "$WORK/newlib/newlib/libc/include/." "$WORK/sysroot/include/"
    cp -r "$WORK/newlib/newlib/libc/machine/aarch64/." "$WORK/sysroot/include/"
    rm -f "$WORK/sysroot/include/asmdefs.h"
    # Generated at newlib build time; values match devkitA64's configuration.
    cat > "$WORK/sysroot/include/newlib.h" <<'H'
#ifndef __NEWLIB_H__
#define __NEWLIB_H__ 1
#include <_newlib_version.h>
#define _WANT_IO_C99_FORMATS 1
#define _WANT_IO_LONG_LONG 1
#define _WANT_REGISTER_FINI 1
#define _MB_LEN_MAX 8
#define HAVE_INITFINI_ARRAY 1
#define _ATEXIT_DYNAMIC_ALLOC 1
#define _HAVE_LONG_DOUBLE 1
#define _HAVE_CC_INHIBIT_LOOP_TO_LIBCALL 1
#define _LDBL_EQ_DBL 1
#define _FVWRITE_IN_STREAMIO 1
#define _FSEEK_OPTIMIZATION 1
#define _WIDE_ORIENT 1
#define _UNBUF_STREAM_OPT 1
#define _RETARGETABLE_LOCKING 1
#endif
H
    printf '#define _NEWLIB_VERSION_H__ 1\n#define _NEWLIB_VERSION "4.4.0"\n#define __NEWLIB__ 4\n#define __NEWLIB_MINOR__ 4\n#define __NEWLIB_PATCHLEVEL__ 0\n' \
        > "$WORK/sysroot/include/_newlib_version.h"
fi

# 2. One translation unit that includes every header.
{
    echo '#include <switch.h>'
    for h in netdb.h poll.h sys/poll.h sys/ioctl.h sys/sockio.h sys/filio.h sys/sysctl.h sys/socket.h \
             netinet/in.h netinet/tcp.h netinet/udp.h arpa/inet.h net/if.h net/if_media.h net/if_dl.h \
             net/if_arp.h net/bpf.h net/dlt.h net/route.h; do
        echo "#include <$h>"
    done
} > "$WORK/all.c"

# 3. clang: AST, record layouts, macro definitions.
CC="$ROOT/tools/cc.sh"
"$CC" -fsyntax-only -Xclang -ast-dump=json "$WORK/all.c" > "$WORK/ast.json"
"$CC" -fsyntax-only -Xclang -fdump-record-layouts-complete "$WORK/all.c" > "$WORK/layouts.txt"
"$CC" -E -dD "$WORK/all.c" > "$WORK/macros.i"

# 4. Generate.
python3 -I "$ROOT/tools/macros.py"
python3 -I "$ROOT/tools/gen.py"
python3 -I "$ROOT/tools/gen_tests.py"
