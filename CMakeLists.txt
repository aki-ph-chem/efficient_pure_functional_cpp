cmake_minimum_required(VERSION 3.13)

# Enable generation of compile commands
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(efficient_pure_functional_cpp)

set(CMAKE_CXX_STANDARD 23)

add_executable(use_copy src_cpp/use_copy.cpp)
