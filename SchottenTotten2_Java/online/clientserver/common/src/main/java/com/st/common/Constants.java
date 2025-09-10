package com.st.common;

import java.util.ArrayList;
import java.util.List;

public class Constants {
    public static final int NUM_WALLS = 7;
    public static final int HAND_SIZE = 6;
    public static final int NUM_CAULDRONS = 2;

    public static final int[] WALL_LENGTHS = {3, 4, 3, 2, 3, 4, 3};
    public static final int[] DAMAGED_WALL_LENGTHS = {3, 2, 3, 4, 3, 2, 3};
    public static final WallPattern[] WALL_PATTERNS = {WallPattern.PLUS, WallPattern.NONE, WallPattern.NONE, WallPattern.NONE, WallPattern.NONE, WallPattern.NONE, WallPattern.MINUS};
    public static final WallPattern[] DAMAGED_WALL_PATTERNS = {WallPattern.RUN, WallPattern.EQUALS, WallPattern.COLOR, WallPattern.MINUS, WallPattern.COLOR, WallPattern.EQUALS, WallPattern.RUN};

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
}