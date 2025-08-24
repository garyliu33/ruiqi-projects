//
// Created by ruiqi on 7/27/2025.
//

#ifndef PLAYER_HPP
#define PLAYER_HPP
#include "ShipBoard.hpp"
#include "TargetBoard.hpp"

class Player {
    ShipBoard shipBoard;
    TargetBoard targetBoard;
    int id;

public:
    explicit Player(int id);

    void placeShips();
    void takeTurn(ShipBoard& opponentBoard);
    ShipBoard& getShipBoard();
    bool hasLost();
};



#endif //PLAYER_HPP
