//
// Created by ruiqi on 7/27/2025.
//

#include "TargetBoard.hpp"

#include "ShipBoard.hpp"

TargetBoard::TargetBoard() {
    board = std::vector(10, std::vector(10, TargetCell()));
}

void TargetBoard::display() {
    std::cout << " ";
    for (int col = 0; col < board[0].size(); col++) {
        std::cout << " " << col;
    }
    std::cout << "\n";
    for (int row = 0; row < board.size(); row++) {
        std::cout << getRowChar(row);
        for (auto & cell : board[row]) {
            std::cout << "|" << cell.displayChar();
        }
        std::cout << "|\n";
    }
}

bool TargetBoard::attack(Point& p, ShipBoard& opponentBoard) {
    TargetCell& cell = board[p.row][p.col];
    if (cell.status == TargetCellStatus::UNKNOWN) {
        if (opponentBoard.hit(p)) {
            cell.setStatus(TargetCellStatus::HIT);
            if (opponentBoard.getShip(p).isSunk()) {
                std::cout << "Sunk!\n";
            } else {
                std::cout << "Hit!\n";
            }
        } else {
            cell.setStatus(TargetCellStatus::MISS);
            std::cout << "Miss.\n";
        }
        return true;
    }
    return false;
}
