import java.util.*;

public class Discard {
    private final Set<Card> discard;

    public Discard() {
        discard = new TreeSet<>();
    }

    public void add(Card card) {
        discard.add(card);
    }

    public void addAll(Collection<Card> cards) {
        discard.addAll(cards);
    }

    public boolean contains(Card card) {
        return discard.contains(card);
    }

    public boolean isEmpty() {
        return discard.isEmpty();
    }

    public void clear() {
        discard.clear();
    }

    public Map<CardColor, List<Card>> getCardsByColor() {
        Map<CardColor, List<Card>> cardsByColor = new TreeMap<>();
        for (CardColor color : CardColor.getAllColors()) {
            cardsByColor.put(color, new ArrayList<>());
        }

        for (Card card : discard) {
            cardsByColor.get(card.getColor()).add(card);
        }

        return cardsByColor;
    }
}
