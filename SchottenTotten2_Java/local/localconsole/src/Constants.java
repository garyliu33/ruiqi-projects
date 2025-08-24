import java.util.*;

public class Constants {
    public static final List<String> colors = List.of("red", "blue", "yellow", "green", "pink");
    public static final List<String> emojis = List.of("♥️", "\uD83D\uDC8E", "⭐️", "\uD83C\uDF40", "\uD83C\uDF38");
    public static final List<String> words = List.of("heart", "diamond", "star", "clover", "flower");
    public static final List<String> fruits = List.of("apple", "grape", "lemon", "mango", "peach");
    public static final List<Integer> values = List.of(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

    public static final int handSize = 6;
    public static final int numCauldrons = 3;
    public static final int numWalls = 7;

    public static final String CAULDRON = "\uD83E\uDDC9";
    public static final String[] leftWalls = {"||", "| ", "  "};
    public static final String[] rightWalls = {"||", " |", "  "};
    public static final String COLOR = "c";
    public static final String RUN = "r";
    public static final String EQUALS = "=";
    public static final String PLUS = "+";
    public static final String MINUS = "-";
    public static final String NONE = " ";

    public static final int attackerWins = 1;
    public static final int defenderWins = 2;
    public static final int noWinner = 0;

    public static final int[] wallLengths = {3, 4, 3, 2, 3, 4, 3};
    public static final int[] damagedWallLengths = {3, 2, 3, 4, 3, 2, 3};
    public static final String[] wallPatterns = {PLUS, NONE, NONE, NONE, NONE, NONE, MINUS};
    public static final String[] damagedWallPatterns = {RUN, EQUALS, COLOR, MINUS, COLOR, EQUALS, RUN};

    public static Set<Card> allCards() {
        Set<Card> result = new TreeSet<>();
        for (String color : colors) {
            for (int value : values) {
                result.add(new Card(color, value));
            }
        }
        return result;
    }

    public static int colorIndex(String str) {
        str = str.trim();
        if (emojis.contains(str)) {
            return emojis.indexOf(str);
        }

        str = str.toLowerCase();
        if (words.contains(str)) {
            return words.indexOf(str);
        }

        if (fruits.contains(str)) {
            return fruits.indexOf(str);
        }

        return colors.indexOf(str);
    }

    public static String convert(String str) {
        return colors.get(colorIndex(str));
    }

    public static String cardSpace() {
        if (Local.useEmojis) {
            return "    ";
        } else {
            return "       ";
        }
    }

    public static int longestWall() {
        int max = 0;
        for (int wall : wallLengths) {
            if (wall > max) {
                max = wall;
            }
        }
        for (int wall : damagedWallLengths) {
            if (wall > max) {
                max = wall;
            }
        }
        return max;
    }
}