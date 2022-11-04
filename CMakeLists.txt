cmake_minimum_required(VERSION 3.23)
project(advent_of_code)

set(CMAKE_CXX_STANDARD 14)

include_directories(day14_cpp)

add_executable(advent_of_code
        day14_cpp/main.cpp
        day14_cpp/Makefile)
