//
// Created by ruiqi on 7/27/2025.
//

#include "ShipCell.hpp"

void ShipCell::setStatus(ShipCellStatus newStatus) {
    status = newStatus;
}

char ShipCell::displayChar() const {
    return static_cast<char>(status);
}
