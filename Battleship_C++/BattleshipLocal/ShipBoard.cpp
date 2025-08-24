//
// Created by ruiqi on 7/27/2025.
//

#include "ShipBoard.hpp"

#include <algorithm>
#include <iostream>

ShipBoard::ShipBoard() {
    board = std::vector(10, std::vector(10, ShipCell()));
}

void ShipBoard::display() {
    std::cout << " ";
    for (int col = 0; col < board[0].size(); col++) {
        std::cout << " " << col;
    }
    std::cout << "\n";
    for (int row = 0; row < board.size(); row++) {
        std::cout << getRowChar(row);
        for (auto & cell : board[row]) {
            std::cout << "|" << cell.displayChar();
        }
        std::cout << "|\n";
    }
}

void ShipBoard::placeShip(int length) {
    while (true) {
        std::cout << "Placing ship of length " << length << ".\n";
        std::cout << "Enter one endpoint: ";
        Point end1 = getPointInput();
        std::cout << "Enter other endpoint: ";
        Point end2 = getPointInput();

        if ((end1.row == end2.row && abs(end1.col - end2.col) == length - 1) || end1.col == end2.col && abs(end1.row - end2.row) == length - 1) {
            std::vector<Point> points = getPointsBetween(end1, end2);
            bool isValid = true;
            for (Point& p : points) {
                if (board[p.row][p.col].status != ShipCellStatus::Empty) {
                    std::cout << "You can't stack ships.\n";
                    isValid = false;
                }
            }

            if (!isValid) {
                continue;
            }

            ships.emplace_back(length, points);
            for (Point& p : points) {
                board[p.row][p.col].setStatus(ShipCellStatus::Intact);
            }
            display();
            return;
        }
        std::cout << "Invalid input.\n";
    }
}

std::vector<Point> ShipBoard::getPointsBetween(Point& p1, Point& p2) {
    std::vector<Point> result;
    if (p1.row == p2.row) {
        for (int col = std::min(p1.col, p2.col); col <= std::max(p1.col, p2.col); col++) {
            result.emplace_back(p1.row, col);
        }
    } else if (p1.col == p2.col) {
        for (int row = std::min(p1.row, p2.row); row <= std::max(p1.row, p2.row); row++) {
            result.emplace_back(row, p1.col);
        }
    } else {
        throw std::invalid_argument("invalid points");
    }
    return result;
}

bool ShipBoard::hit(Point &p) {
    ShipCell cell = board[p.row][p.col];
    if (cell.status == ShipCellStatus::Empty) {
        cell.setStatus(ShipCellStatus::Attacked);
        return false;
    }
    if (cell.status == ShipCellStatus::Intact) {
        cell.setStatus(ShipCellStatus::Damaged);
        getShip(p).registerHit();
        return true;
    }
    throw std::invalid_argument("already attacked there");
}

Ship& ShipBoard::getShip(Point &p) {
    for (Ship& ship : ships) {
        if (ship.occupies(p)) {
            return ship;
        }
    }
    throw std::invalid_argument("no ship here");
}

bool ShipBoard::allSunk() {
    for (Ship& ship : ships) {
        if (!ship.isSunk()) {
            return false;
        }
    }
    return true;
}