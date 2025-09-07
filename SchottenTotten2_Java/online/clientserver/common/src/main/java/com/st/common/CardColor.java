package com.st.common;

import java.awt.Color;

import com.st.proto.Card.ColorProto;

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

    public ColorProto toProto() {
        switch (this) {
            case CardColor.RED -> {
                return ColorProto.RED;
            }
            case CardColor.BLUE -> {
                return ColorProto.BLUE;
            }
            case CardColor.YELLOW -> {
                return ColorProto.YELLOW;
            }
            case CardColor.GREEN -> {
                return ColorProto.GREEN;
            }
            case CardColor.GRAY -> {
                return ColorProto.GRAY;
            }
            case CardColor.ACTION_COLOR -> {
                return ColorProto.ACTION;
            }
        }
        throw new AssertionError();
    }

    public static CardColor fromProto(ColorProto proto) {
        switch (proto) {
            case ColorProto.RED -> {
                return CardColor.RED;
            }
            case ColorProto.BLUE -> {
                return CardColor.BLUE;
            }
            case ColorProto.YELLOW -> {
                return CardColor.YELLOW;
            }
            case ColorProto.GREEN -> {
                return CardColor.GREEN;
            }
            case ColorProto.GRAY -> {
                return CardColor.GRAY;
            }
            case ColorProto.ACTION -> {
                return CardColor.ACTION_COLOR;
            }
        }
        throw new AssertionError();
    }
}
