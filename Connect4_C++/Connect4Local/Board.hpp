//
// Created by ruiqi on 7/27/2025.
//

#ifndef BOARD_HPP
#define BOARD_HPP
#include <vector>

enum class Symbol {
    X = 'X', O = 'O', EMPTY = '_'
};

class Board {
    static constexpr int ROWS = 6;
    static constexpr int COLS = 7;
    std::vector<std::vector<Symbol>> board = std::vector(ROWS, std::vector(COLS, Symbol::EMPTY));

public:
    void display() const;
    Symbol getWinner() const;
    bool playPiece(Symbol piece, int col);
};

#endif //BOARD_HPP
