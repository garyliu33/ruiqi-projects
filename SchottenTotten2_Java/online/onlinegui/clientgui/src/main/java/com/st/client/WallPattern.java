package com.st.client;

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
}
