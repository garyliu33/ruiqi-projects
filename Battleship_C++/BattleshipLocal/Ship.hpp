//
// Created by ruiqi on 7/27/2025.
//

#ifndef SHIP_HPP
#define SHIP_HPP
#include <vector>

#include "Point.hpp"

class Ship {
    int length;
    int hits;
    std::vector<Point> occupiedPoints;

public:
    explicit Ship(int len, std::vector<Point> points);

    void registerHit();
    [[nodiscard]] bool isSunk() const;
    [[nodiscard]] bool occupies(Point& p) const;
};



#endif //SHIP_HPP
