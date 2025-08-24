import java.util.*;

public class Hand {
    private final Set<Card> cards;

    public Hand() {
        cards = new TreeSet<>();
    }

    public Set<Card> getCards() {
        return cards;
    }

    public void add(Card card) {
        cards.add(card);
    }

    public void remove(Card card) {
        cards.remove(card);
    }
}
