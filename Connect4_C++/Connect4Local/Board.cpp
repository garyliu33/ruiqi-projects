//
// Created by ruiqi on 7/27/2025.
//

#include "Board.hpp"

#include <iostream>

void Board::display() const {
    for (int row = 0; row < ROWS; row++) {
        for (int col = 0; col < COLS; col++) {
            std::cout << "|" << static_cast<char>(board[row][col]);
        }
        std::cout << "|\n";
    }
    for (int col = 1; col <= COLS; col++) {
        std::cout << " " << col;
    }
    std::cout << " \n";
}

Symbol Board::getWinner() const {
    for (int row = 0; row <= ROWS - 4; row++) {
        for (int col = 0; col < COLS; col++) {
            Symbol symbol = board[row][col];
            if (symbol != Symbol::EMPTY && symbol == board[row + 1][col] && symbol == board[row + 2][col] && symbol == board[row + 3][col]) {
                return symbol;
            }
        }
    }

    for (int row = 0; row < ROWS; row++) {
        for (int col = 0; col <= COLS - 4; col++) {
            Symbol symbol = board[row][col];
            if (symbol != Symbol::EMPTY && symbol == board[row][col + 1] && symbol == board[row][col + 2] && symbol == board[row][col + 3]) {
                return symbol;
            }
        }
    }

    for (int row = 0; row <= ROWS - 4; row++) {
        for (int col = 0; col <= COLS - 4; col++) {
            Symbol symbol = board[row][col];
            if (symbol != Symbol::EMPTY && symbol == board[row + 1][col + 1] && symbol == board[row + 2][col + 2] && symbol == board[row + 3][col + 3]) {
                return symbol;
            }
        }
    }

    for (int row = 3; row < ROWS; row++) {
        for (int col = 0; col <= COLS - 4; col++) {
            Symbol symbol = board[row][col];
            if (symbol != Symbol::EMPTY && symbol == board[row - 1][col + 1] && symbol == board[row - 2][col + 2] && symbol == board[row - 3][col + 3]) {
                return symbol;
            }
        }
    }

    return Symbol::EMPTY;
}

bool Board::playPiece(Symbol piece, int col) {
    for (int i = ROWS - 1; i >= 0; i--) {
        if (board[i][col - 1] == Symbol::EMPTY) {
            board[i][col - 1] = piece;
            return true;
        }
    }
    return false;
}