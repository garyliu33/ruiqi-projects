//
// Created by ruiqi on 7/27/2025.
//

#ifndef TARGETBOARD_HPP
#define TARGETBOARD_HPP
#include <vector>

#include "Point.hpp"
#include "ShipBoard.hpp"
#include "TargetCell.hpp"

class TargetBoard {
    std::vector<std::vector<TargetCell>> board;

public:
    TargetBoard();

    void display();
    bool attack(Point& p, ShipBoard& opponentBoard);
};

#endif //TARGETBOARD_HPP
