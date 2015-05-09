#!/bin/bash

bison glslang/MachineIndependent/glslang.y --no-lines \
    --defines=glslang/gen/glslang_tab.cpp.h \
    --output-file=glslang/gen/glslang_tab.cpp
