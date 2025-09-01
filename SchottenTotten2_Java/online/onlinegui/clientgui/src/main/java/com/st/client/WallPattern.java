package com.st.client;

import com.st.proto.Wall.WallPatternProto;

public enum WallPattern {
    COLOR("C"),
    RUN("R"),
    EQUALS("="),
    PLUS("+"),
    MINUS("-"),
    NONE(" ");
    private final String symbol;

    WallPattern(String symbol) {
        this.symbol = symbol;
    }

    public String getSymbol() {
        return symbol;
    }

    public static WallPattern fromProto(WallPatternProto proto) {
        switch (proto) {
            case WallPatternProto.COLOR -> {
                return WallPattern.COLOR;
            }
            case WallPatternProto.RUN -> {
                return WallPattern.RUN;
            }
            case WallPatternProto.EQUALS -> {
                return WallPattern.EQUALS;
            }
            case WallPatternProto.PLUS -> {
                return WallPattern.PLUS;
            }
            case WallPatternProto.MINUS -> {
                return WallPattern.MINUS;
            }
            case WallPatternProto.NONE_PATTERN -> {
                return WallPattern.NONE;
            }
        }
        throw new AssertionError();
    }
}
