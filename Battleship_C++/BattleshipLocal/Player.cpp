//
// Created by ruiqi on 7/27/2025.
//

#include "Player.hpp"

Player::Player(int id) : id(id) {}

void Player::placeShips() {
    std::cout << "Place your ships. (lengths: 5, 4, 3, 3, 2)\n";
    shipBoard.display();
    shipBoard.placeShip(5);
    shipBoard.placeShip(4);
    shipBoard.placeShip(3);
    shipBoard.placeShip(3);
    shipBoard.placeShip(2);
}

void Player::takeTurn(ShipBoard& opponentBoard) {
    while (true) {
        std::cout << "Player " << id << "'s turn.\n";
        targetBoard.display();
        std::cout << "\n";
        shipBoard.display();
        std::cout << "Enter target: ";
        if (const Point p = getPointInput(); targetBoard.attack(p, opponentBoard)) {
            return;
        }
        std::cout << "Invalid move.\n";
    }
}

ShipBoard& Player::getShipBoard() {
    return shipBoard;
}

bool Player::hasLost() {
    return shipBoard.allSunk();
}
