import java.util.*;

public class Hand {
    private final Set<Card> cards;

    public Hand() {
        cards = new TreeSet<>();
    }

    public Set<Card> getCards() {
        return cards;
    }

    public int size() {
        return cards.size();
    }

    public void add(Card card) {
        cards.add(card);
    }

    public boolean remove(Card card) {
        return cards.remove(card);
    }
}
