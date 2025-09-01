package com.st.client;

import com.st.proto.GameState.WinnerProto;

public enum Winner {
    ATTACKER, DEFENDER, NONE;

    public static Winner fromProto(WinnerProto proto) {
        switch (proto) {
            case WinnerProto.ATTACKER -> {
                return Winner.ATTACKER;
            }
            case WinnerProto.DEFENDER -> {
                return Winner.DEFENDER;
            }
            case WinnerProto.NONE_WINNER -> {
                return Winner.NONE;
            }
        }
        throw new AssertionError();
    }
}