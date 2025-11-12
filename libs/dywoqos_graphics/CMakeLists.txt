cmake_minimum_required(VERSION 4.1.2)
project(dywoqos_graphics CXX)

set(CMAKE_CXX_STANDARD 23)

file(GLOB GRAPHICS_SOURCES src/*.cxx)

# Library
add_library(${PROJECT_NAME} STATIC ${GRAPHICS_SOURCES})
target_include_directories(${PROJECT_NAME}
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
)

# Testing
enable_testing()

# GTest
find_package(GTest REQUIRED)

# Test executable
add_executable(my_tests tests/test_example.cxx)
target_link_libraries(my_tests PRIVATE ${PROJECT_NAME} GTest::GTest GTest::Main)

# Register the test
add_test(NAME DywoqosGraphics_Drawing COMMAND my_tests)
set_tests_properties(DywoqosGraphics_Drawing PROPERTIES
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/tests
)
