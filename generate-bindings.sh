#!/bin/bash

IN="extern/glslang/glslang/Public/ShaderLang.h"
OUT="src/lib.rs"

bindgen -o "$OUT" "$IN"

sed -i '1i\
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)] \
extern crate libc;
' "$OUT"
