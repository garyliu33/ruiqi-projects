import java.util.*;

public class Constants {
    public static final List<Integer> VALUES = List.of(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    public static final Set<Card> ALL_CARDS;

    public static final int NUM_WALLS;
    public static final int[] WALL_LENGTHS = {3, 4, 3, 2, 3, 4, 3};
    public static final int[] DAMAGED_WALL_LENGTHS = {3, 2, 3, 4, 3, 2, 3};
    public static final WallPattern[] WALL_PATTERNS = {WallPattern.PLUS, WallPattern.NONE, WallPattern.NONE, WallPattern.NONE, WallPattern.NONE, WallPattern.NONE, WallPattern.MINUS};
    public static final WallPattern[] DAMAGED_WALL_PATTERNS = {WallPattern.RUN, WallPattern.EQUALS, WallPattern.COLOR, WallPattern.MINUS, WallPattern.COLOR, WallPattern.EQUALS, WallPattern.RUN};

    public static final int NUM_CAULDRONS = 3;
    public static final int HAND_SIZE = 6;

    public static final int WINDOW_WIDTH = 1280;
    public static final int WINDOW_HEIGHT = 720;
    public static final int CARD_WIDTH = WINDOW_WIDTH / 20;
    public static final int CARD_HEIGHT = WINDOW_HEIGHT / 8;
    public static final int OVERLAP = CARD_HEIGHT / 3;


    static {
        ALL_CARDS = new TreeSet<>();
        for (CardColor cardColor : CardColor.getAllColors()) {
            for (int value : VALUES) {
                ALL_CARDS.add(new Card(cardColor, value));
            }
        }

        NUM_WALLS = WALL_LENGTHS.length;
    }
}
