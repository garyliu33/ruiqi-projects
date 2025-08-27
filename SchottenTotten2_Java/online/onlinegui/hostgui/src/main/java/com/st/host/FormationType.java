package com.st.host;

public enum FormationType {
    COLOR_RUN(4),
    SAME_STRENGTH(3),
    COLOR(2),
    RUN(1),
    SUM(0);
    private final int strength;

    FormationType(int strength) {
        this.strength = strength;
    }

    public int getStrength() {
        return strength;
    }
}
