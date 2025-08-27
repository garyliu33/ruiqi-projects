package com.st.host;

import java.awt.*;

public enum CardColor {
    RED(new Color(200, 46, 46)),
    BLUE(new Color(61, 165, 209)),
    YELLOW(new Color(222, 195, 76)),
    GREEN(new Color(63, 175, 55)),
    GRAY(new Color(110, 110, 110)),
    ACTION_COLOR(Color.BLACK);

    private final Color color;

    CardColor(Color color) {
        this.color = color;
    }

    public Color getDisplayColor() {
        return color;
    }

    public static CardColor[] getAllColors() {
        return new CardColor[] {
            RED, BLUE, YELLOW, GREEN, GRAY
        };
    }
}
