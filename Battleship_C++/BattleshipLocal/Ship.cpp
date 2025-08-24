//
// Created by ruiqi on 7/27/2025.
//

#include "Ship.hpp"

#include <algorithm>
#include <cassert>
#include <utility>

Ship::Ship(int len, std::vector<Point> points) {
    assert(len == points.size());
    length = len;
    hits = 0;
    occupiedPoints = std::move(points);
}

void Ship::registerHit() {
    hits++;
}

bool Ship::isSunk() const {
    return hits == length;
}

bool Ship::occupies(Point &p) const {
    return std::ranges::count(occupiedPoints, p) > 0;
}
