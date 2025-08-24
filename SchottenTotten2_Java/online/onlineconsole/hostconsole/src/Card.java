import java.util.*;

public class Card implements Comparable<Card> {
    private final int value;
    private final Color color;
    private static final int ACTION_VALUE = -1;

    public static final Card ACTION = new Card(null, ACTION_VALUE);

    public Card(Color color, int value) {
        this.value = value;
        this.color = color;
    }

    public Card(String name) {
        this.color = Color.convert(name.substring(0, name.length() - 2));
        this.value = Integer.parseInt(name.substring(name.length() - 2));
    }

    public String toString() {
        return value <= 9 ? color.getSymbol() + "0" + value : color.getSymbol() + value;
    }

    public int getValue() {
        return value;
    }

    public Color getColor() {
        return color;
    }

    public int compareTo(Card other) {
        if (this.color == other.color) {
            return this.value - other.value;
        }
        return this.color.compareTo(other.color);
    }

    public boolean equals(Object o) {
        if (this == o) {
            return true;
        } else if (o instanceof Card) {
            return this.color == ((Card) o).color && this.value == ((Card) o).value;
        } else {
            return false;
        }
    }

    public int hashCode() {
        return Objects.hash(value, color);
    }

    public static boolean isValid(String name) {
        if (name.length() <= 2) {
            return false;
        }

        Color color = Color.convert(name.substring(0, name.length() - 2));
        if (color == null) {
            return false;
        }

        try {
            return Constants.VALUES.contains(Integer.parseInt(name.substring(name.length() - 2)));
        } catch (NumberFormatException e) {
            return false;
        }
    }

    public boolean isAction() {
        return value == ACTION_VALUE;
    }
}