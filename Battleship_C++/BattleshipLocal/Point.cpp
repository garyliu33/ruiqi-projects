//
// Created by ruiqi on 7/27/2025.
//

#include "Point.hpp"

#include <stdexcept>
#include <cassert>

Point::Point() : row(0), col(0) {}

Point::Point(int row, int col) : row(row), col(col) {}

Point stringToPoint(std::string &str) {
    assert(str.length() == 2);
    Point p;

    int r = std::toupper(str[0]) - 'A';
    if (r < 0 || r > 9) {
        throw std::invalid_argument("Invalid row letter");
    }
    p.row = r;

    int col = std::stoi(str.substr(1));
    if (col < 0 || col > 9) {
        throw std::invalid_argument("Invalid column number");
    }
    p.col = col;

    return p;
}

bool isValid(std::string &str) {
    if (str.size() != 2) {
        return false;
    }
    int r = std::toupper(str[0]) - 'A';
    if (r < 0 || r > 9) {
        return false;
    }

    int number = std::stoi(str.substr(1));
    if (number < 0 || number > 9) {
        return false;
    }
    return true;
}

char getRowChar(int rowIndex) {
    return static_cast<char>(rowIndex + 'A');
}

Point getPointInput()  {
    while (true) {
        std::string str;
        std::cin >> str;
        if (isValid(str)) {
            return stringToPoint(str);
        }
    }
}