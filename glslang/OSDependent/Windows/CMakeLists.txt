cmake_minimum_required(VERSION 2.8)

include_directories(. ../../../OGLCompilersDLL)

set(SOURCES ossource.cpp osinclude.h)

if(WIN32)
    source_group("Source" FILES ${SOURCES})
endif(WIN32)
