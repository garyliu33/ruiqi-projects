import java.util.*;

public class Deck {
    private static Deck instance;
    private final Deque<Card> deck;

    private Deck() {
        deck = new ArrayDeque<>();
        deck.addAll(Constants.ALL_CARDS);
    }

    public static synchronized Deck getInstance() {
        if (instance == null) {
            instance = new Deck();
        }
        return instance;
    }

    public synchronized int size() {
        return deck.size();
    }

    public synchronized boolean isEmpty() {
        return deck.isEmpty();
    }

    public synchronized void shuffle() {
        List<Card> cards = new ArrayList<>(deck);
        Collections.shuffle(cards);
        deck.clear();
        deck.addAll(cards);
    }

    public synchronized Card pop() {
        if (deck.isEmpty()) {
            return null;
        }
        return deck.pop();
    }

    public synchronized void reset() {
        deck.clear();
        deck.addAll(Constants.ALL_CARDS);
        shuffle();
    }
}
