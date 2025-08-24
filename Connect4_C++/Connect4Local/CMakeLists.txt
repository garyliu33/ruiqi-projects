cmake_minimum_required(VERSION 3.31)
project(Connect4Local)

set(CMAKE_CXX_STANDARD 20)

add_executable(Connect4Local main.cpp
        Board.cpp
        Board.hpp)
