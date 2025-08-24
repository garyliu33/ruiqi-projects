public enum WallPattern {
    COLOR("c"),
    RUN("r"),
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
