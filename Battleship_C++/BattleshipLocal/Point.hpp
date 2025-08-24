//
// Created by ruiqi on 7/27/2025.
//

#ifndef POINT_HPP
#define POINT_HPP
#include <iostream>
#include <string>

struct Point {
    int row;
    int col;

    Point();
    Point(int row, int col);

    bool operator==(const Point& other) const {
        return row == other.row && col == other.col;
    }
};

Point stringToPoint(std::string &str);
char getRowChar(int row);
Point getPointInput();
bool isValid(std::string &str);

#endif //POINT_HPP
