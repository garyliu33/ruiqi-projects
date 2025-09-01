package com.st.common;

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