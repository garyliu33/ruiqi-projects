package com.st.common;

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
