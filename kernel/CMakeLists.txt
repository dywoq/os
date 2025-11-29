cmake_minimum_required(VERSION 3.28.3)

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Debug CACHE STRING "Choose the type of build (Debug or Release)" FORCE)
endif()

set(VALID_BUILD_TYPES Debug Release)
if(NOT CMAKE_BUILD_TYPE IN_LIST VALID_BUILD_TYPES)
    message(FATAL_ERROR "Invalid CMAKE_BUILD_TYPE: ${CMAKE_BUILD_TYPE}. Allowed: ${VALID_BUILD_TYPES}")
endif()

project(DywoqOS LANGUAGES C)

# ------------------------------------------------------------------
# Compiler and base flags
# ------------------------------------------------------------------
set(CMAKE_C_COMPILER gcc-13)

add_compile_options(
    -fno-stack-protector
    -fpic
    -fshort-wchar
    -mno-red-zone
    -ffreestanding
    -Wall
    -Wextra
)

# ------------------------------------------------------------------
# Build-type specific defines
# ------------------------------------------------------------------
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    add_compile_definitions(BUILD_DEBUG=1 BUILD_RELEASE=0)
else() # Release
    add_compile_definitions(BUILD_DEBUG=0 BUILD_RELEASE=1)
endif()

# ------------------------------------------------------------------
# Gather all source files
# ------------------------------------------------------------------
file(GLOB_RECURSE SOURCES
    "src/*.c"
    "src/kernel/*.c"
)

add_library(dywoqos SHARED ${SOURCES})

# UEFI-specific linker flags
set_target_properties(dywoqos PROPERTIES
    OUTPUT_NAME "dywoqos"
    SUFFIX      ".efi"
    PREFIX      ""

    LINK_FLAGS "-nostdlib -znocombreloc -shared -Bsymbolic -T ${CMAKE_SOURCE_DIR}/linker.ld"
)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
