#include <iostream>

#include "Board.hpp"

int getCol() {
    while (true) {
        std::cout << "Enter column (1-7): ";
        int col;
        std::cin >> col;
        if (col >= 1 && col <= 7) {
            return col;
        }
    }
}

void takeTurn(Board* board, Symbol piece) {
    while (true) {
        if (int col = getCol(); board->playPiece(piece, col)) {
            board->display();
            return;
        }
    }
}

int main() {
    auto board = Board();
    board.display();
    while (true) {
        takeTurn(&board, Symbol::X);
        Symbol winner = board.getWinner();
        if (winner == Symbol::X) {
            std::cout << "X wins!";
            break;
        }
        if (winner == Symbol::O) {
            std::cout << "O wins!";
            break;
        }

        takeTurn(&board, Symbol::O);
        winner = board.getWinner();
        if (winner == Symbol::X) {
            std::cout << "X wins!";
            break;
        }
        if (winner == Symbol::O) {
            std::cout << "O wins!";
            break;
        }
    }

    return 0;
}
