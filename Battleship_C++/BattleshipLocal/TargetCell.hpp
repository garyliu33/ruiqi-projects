//
// Created by ruiqi on 7/27/2025.
//

#ifndef TARGETCELL_HPP
#define TARGETCELL_HPP

enum class TargetCellStatus {
    UNKNOWN = '?', HIT = 'X', MISS = 'O'
};

struct TargetCell {
    TargetCellStatus status;

    TargetCell() : status(TargetCellStatus::UNKNOWN) {}

    void setStatus(TargetCellStatus newStatus);
    char displayChar() const;
};

#endif //TARGETCELL_HPP
