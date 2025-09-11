package com.st.common;

import java.util.ArrayList;
import java.util.List;

public class Constants {
    public static final int NUM_WALLS = 7;
    public static final int HAND_SIZE = 6;
    public static final int NUM_CAULDRONS = 2;

    public static final List<Integer> VALUES = List.of(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

    public static final List<Card> ALL_CARDS = new ArrayList<>();

    static {
        for (CardColor color : CardColor.getAllColors()) {
            for (int value : VALUES) {
                ALL_CARDS.add(new Card(color, value));
            }
        }
    }

    // Default UI sizes, can be ignored by server
    public static int WINDOW_WIDTH = 1280;
    public static int WINDOW_HEIGHT = 720;
    public static int CARD_WIDTH = 70;
    public static int CARD_HEIGHT = 100;
    public static int CARD_FONT_SIZE = 20;
    public static int WALL_WIDTH = CARD_WIDTH;
    public static int WALL_LABEL_HEIGHT = 20;
    public static int OVERLAP = 15;
    public static int POP_OFFSET = 15;
    public static int WALL_OVERALL_HEIGHT = 2 * (CARD_HEIGHT + 3 * OVERLAP) + WALL_LABEL_HEIGHT;

    public static void resize(int width, int height) {
        WINDOW_WIDTH = width;
        WINDOW_HEIGHT = height;
        CARD_WIDTH = (int) (width * 0.0546875);
        CARD_HEIGHT = (int) (height * 0.13888888);
        CARD_FONT_SIZE = (int) (CARD_HEIGHT * 0.2);
        WALL_WIDTH = CARD_WIDTH;
        WALL_LABEL_HEIGHT = (int) (height * 0.0277777);
        OVERLAP = (int) (CARD_HEIGHT * 0.15);
        POP_OFFSET = OVERLAP;
        WALL_OVERALL_HEIGHT = 2 * (CARD_HEIGHT + 3 * OVERLAP) + WALL_LABEL_HEIGHT;
    }
}