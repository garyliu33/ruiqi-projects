#include <iostream>

#include "Player.hpp"

int main() {
    Player player1{1};
    Player player2{2};
    player1.placeShips();
    player2.placeShips();

    while (true) {
        player1.takeTurn(player2.getShipBoard());
        if (player2.hasLost()) {
            std::cout << "Player 1 wins!";
            break;
        }
        player2.takeTurn(player1.getShipBoard());
        if (player1.hasLost()) {
            std::cout << "Player 2 wins!";
            break;
        }
    }
    return 0;
}
