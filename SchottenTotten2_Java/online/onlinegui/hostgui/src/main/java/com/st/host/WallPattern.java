package com.st.host;

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

    public WallPatternProto toProto() {
        switch (this) {
            case WallPattern.COLOR -> {
                return WallPatternProto.COLOR;
            }
            case WallPattern.RUN -> {
                return WallPatternProto.RUN;
            }
            case WallPattern.EQUALS -> {
                return WallPatternProto.EQUALS;
            }
            case WallPattern.PLUS -> {
                return WallPatternProto.PLUS;
            }
            case WallPattern.MINUS -> {
                return WallPatternProto.MINUS;
            }
            case WallPattern.NONE -> {
                return WallPatternProto.NONE_PATTERN;
            }
        }
        throw new AssertionError();
    }
}
