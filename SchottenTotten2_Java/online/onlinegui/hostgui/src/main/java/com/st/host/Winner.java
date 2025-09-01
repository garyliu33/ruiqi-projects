package com.st.host;

import com.st.proto.GameState.WinnerProto;

public enum Winner {
    ATTACKER, DEFENDER, NONE;

    public WinnerProto toProto() {
        switch (this) {
            case Winner.ATTACKER -> {
                return WinnerProto.ATTACKER;
            }
            case Winner.DEFENDER -> {
                return WinnerProto.DEFENDER;
            }
            case Winner.NONE -> {
                return WinnerProto.NONE_WINNER;
            }
        }
        throw new AssertionError();
    }
}