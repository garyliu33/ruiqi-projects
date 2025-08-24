import java.util.*;

public class Constants {
    public static final List<Integer> VALUES = List.of(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

    public static final int HAND_SIZE = 6;
    public static final int NUM_CAULDRONS = 3;

    public static final int[] WALL_LENGTHS = {3, 4, 3, 2, 3, 4, 3};
    public static final int[] DAMAGED_WALL_LENGTHS = {3, 2, 3, 4, 3, 2, 3};
    public static final WallPattern[] WALL_PATTERNS = {WallPattern.PLUS, WallPattern.NONE, WallPattern.NONE, WallPattern.NONE, WallPattern.NONE, WallPattern.NONE, WallPattern.MINUS};
    public static final WallPattern[] DAMAGED_WALL_PATTERNS = {WallPattern.RUN, WallPattern.EQUALS, WallPattern.COLOR, WallPattern.MINUS, WallPattern.COLOR, WallPattern.EQUALS, WallPattern.RUN};

    public static final Set<Card> ALL_CARDS;
    public static final int LONGEST_WALL;
    public static final String CAULDRON = "\uD83E\uDDC9";
    public static final String CARD_SPACE;
    public static final int NUM_WALLS;

    static {
        ALL_CARDS = new TreeSet<>();
        for (Color color : Color.values()) {
            for (int value : VALUES) {
                ALL_CARDS.add(new Card(color, value));
            }
        }

        int max = 0;
        for (int wall : WALL_LENGTHS) {
            if (wall > max) {
                max = wall;
            }
        }
        for (int wall : DAMAGED_WALL_LENGTHS) {
            if (wall > max) {
                max = wall;
            }
        }
        LONGEST_WALL = max;
        CARD_SPACE = Host.useEmojis ? "    " : "       ";
        NUM_WALLS = WALL_LENGTHS.length;
    }
}