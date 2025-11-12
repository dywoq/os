cmake_minimum_required(VERSION 3.28.3)
project(dywoqos_libs)

set(CMAKE_CXX_STANDARD 23)

# Graphics library
file(GLOB GRAPHICS_SOURCES dywoqos_graphics/*.cxx)
target_sources(dywoqos_graphics ${GRAPHICS_SOURCES})
