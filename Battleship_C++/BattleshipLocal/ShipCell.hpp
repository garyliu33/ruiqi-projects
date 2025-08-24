//
// Created by ruiqi on 7/27/2025.
//

#ifndef SHIPCELL_HPP
#define SHIPCELL_HPP

enum class ShipCellStatus {
    Empty = '-', Damaged = 'X', Intact = 'O', Attacked = '-'
};

struct ShipCell {
    ShipCellStatus status;

    ShipCell() : status(ShipCellStatus::Empty) {}

    void setStatus(ShipCellStatus newStatus);
    char displayChar() const;
};

#endif //SHIPCELL_HPP
