import java.util.Objects;

public class Card implements Comparable<Card> {
    private final int value;
    private final CardColor cardColor;

    public static final Card RETREAT = new Card(CardColor.ACTION_COLOR, -1);
    public static final Card CAULDRON = new Card(CardColor.ACTION_COLOR, -2);

    public Card(CardColor cardColor, int value) {
        this.value = value;
        this.cardColor = cardColor;
    }

    public CardColor getColor() {
        return cardColor;
    }

    public int getValue() {
        return value;
    }

    public int compareTo(Card other) {
        if (this.cardColor == other.cardColor) {
            return this.value - other.value;
        }
        return this.cardColor.compareTo(other.cardColor);
    }

    public boolean equals(Object o) {
        if (this == o) {
            return true;
        } else if (o instanceof Card) {
            return this.cardColor == ((Card) o).cardColor && this.value == ((Card) o).value;
        } else {
            return false;
        }
    }

    public int hashCode() {
        return Objects.hash(value, cardColor);
    }
}
