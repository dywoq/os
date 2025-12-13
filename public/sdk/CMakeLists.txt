cmake_minimum_required(VERSION 3.28.3)
project(dywoqos_sdk)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED True)

file(GLOB SOURCES src/*.c)

add_library(${PROJECT_NAME} STATIC ${SOURCES})
target_compile_options(${PROJECT_NAME} PRIVATE -ffreestanding -nostdlib)
