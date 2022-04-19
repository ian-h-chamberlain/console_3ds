#!/usr/bin/env bash

set -euxo pipefail

bindgen "src/ffi/types.h" \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --no-doc-comments \
    --no-layout-tests \
    --ctypes-prefix "::libc" \
    --no-prepend-enum-name \
    --size_t-is-usize \
    --generate "functions,types,vars" \
    --allowlist-type "devoptab_t|_reent|DIR_ITER|_mbstate_t" \
    --opaque-type "_mbstate_t|__locale_t|_Bigint|__tm|_on_exit_args|_atexit|__sbuf|__sFILE|_glue|_rand48|timeval|timespec|stat.*|DIR_ITER" \
    --no-default "__reent" \
    --allowlist-var "STD_.*|^_IO.*|devoptab_list" \
    --allowlist-function "__getreent|__errno" \
    --with-derive-default \
    -- \
    --target=arm-none-eabi \
    --sysroot=$DEVKITARM/arm-none-eabi \
    -isystem$DEVKITARM/arm-none-eabi/include \
    -I$DEVKITPRO/libctru/include \
    -mfloat-abi=hard \
    -march=armv6k \
    -mtune=mpcore \
    -mfpu=vfp \
    -DARM11 \
    -D__3DS__ \
> src/ffi/generated.rs

