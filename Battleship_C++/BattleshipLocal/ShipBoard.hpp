//
// Created by ruiqi on 7/27/2025.
//

#ifndef SHIPBOARD_HPP
#define SHIPBOARD_HPP
#include "Ship.hpp"
#include "ShipCell.hpp"

class ShipBoard {
    std::vector<Ship> ships;
    std::vector<std::vector<ShipCell>> board;
    static std::vector<Point> getPointsBetween(Point& p1, Point& p2);

public:
    ShipBoard();

    Ship& getShip(Point& p);
    void display();
    void placeShip(int length);
    bool hit(Point& p);
    bool allSunk();
};



#endif //SHIPBOARD_HPP
