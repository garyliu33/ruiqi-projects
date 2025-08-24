import java.awt.*;
import java.util.List;
import java.util.Set;
import java.util.TreeSet;

public class Constants {
    public static final List<Integer> VALUES = List.of(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    public static final Set<Card> ALL_CARDS;
    public static final int NUM_WALLS = 7;

    public static int WINDOW_WIDTH;
    public static int WINDOW_HEIGHT;
    public static int CARD_WIDTH;
    public static int CARD_HEIGHT;
    public static int OVERLAP;
    public static int CARD_FONT_SIZE;
    public static int POP_OFFSET;

    public static int WALL_WIDTH;
    public static int WALL_LABEL_HEIGHT;
    public static int WALL_OVERALL_HEIGHT;

    static {
        ALL_CARDS = new TreeSet<>();
        for (CardColor cardColor : CardColor.getAllColors()) {
            for (int value : VALUES) {
                ALL_CARDS.add(new Card(cardColor, value));
            }
        }

        int dpi = Toolkit.getDefaultToolkit().getScreenResolution();
        double scale = dpi / 96.0;

        WINDOW_WIDTH = (int) (1024 * scale);
        WINDOW_HEIGHT = (int) (576 * scale);
        recalculate();
    }

    public static void resize(int newWidth, int newHeight) {
        WINDOW_WIDTH = newWidth;
        WINDOW_HEIGHT = newHeight;
        recalculate();
    }

    private static void recalculate() {
        CARD_WIDTH = WINDOW_WIDTH / 20;
        CARD_HEIGHT = WINDOW_HEIGHT / 8;
        OVERLAP = CARD_HEIGHT / 3;
        CARD_FONT_SIZE = 2 * OVERLAP / 3 - 2;
        POP_OFFSET = OVERLAP / 2;

        WALL_WIDTH = CARD_WIDTH;
        WALL_LABEL_HEIGHT = CARD_HEIGHT / 2;
        WALL_OVERALL_HEIGHT = WINDOW_HEIGHT - 3 * CARD_HEIGHT;
    }
}
