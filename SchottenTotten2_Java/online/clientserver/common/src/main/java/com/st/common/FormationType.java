package com.st.common;

public enum FormationType {
    SUM(0),
    RUN(1),
    COLOR(2),
    SAME_STRENGTH(3),
    COLOR_RUN(4);

    private final int strength;

    FormationType(int strength) {
        this.strength = strength;
    }

    public int getStrength() {
        return strength;
    }
}
