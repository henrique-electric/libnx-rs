#!/bin/bash
# clang configured like devkitA64 for the Switch: aarch64, newlib headers, libnx headers.
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SYSROOT="$ROOT/target/libnx-gen/sysroot/include"
exec clang --target=aarch64-none-elf -D__SWITCH__ -nostdinc -fno-builtin \
    -I"$ROOT/include" -isystem "$SYSROOT" -isystem "$(clang -print-resource-dir)/include" "$@"
