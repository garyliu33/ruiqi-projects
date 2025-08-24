cmake_minimum_required(VERSION 3.31)
project(BattleshipLocal)

set(CMAKE_CXX_STANDARD 20)

add_executable(BattleshipLocal main.cpp
        ShipBoard.cpp
        ShipBoard.hpp
        Point.cpp
        Point.hpp
        ShipCell.cpp
        ShipCell.hpp
        TargetCell.cpp
        TargetCell.hpp
        Ship.cpp
        Ship.hpp
        Player.cpp
        Player.hpp
        TargetBoard.cpp
        TargetBoard.hpp
)
