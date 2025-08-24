//
// Created by ruiqi on 7/27/2025.
//

#include "TargetCell.hpp"

void TargetCell::setStatus(TargetCellStatus newStatus) {
    status = newStatus;
}

char TargetCell::displayChar() const {
    return static_cast<char>(status);
}