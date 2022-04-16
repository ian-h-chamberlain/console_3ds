#!/usr/bin/env bash

set -euxo pipefail


bindgen "$DEVKITARM/arm-none-eabi/include/sys/iosupport.h" \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --no-doc-comments \
    --no-layout-tests \
    --ctypes-prefix "::libc" \
    --no-prepend-enum-name \
    --generate "types" \
    --allowlist-type "devoptab_t" \
    --allowlist-var "STD_.*" \
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
> src/devoptab.rs

