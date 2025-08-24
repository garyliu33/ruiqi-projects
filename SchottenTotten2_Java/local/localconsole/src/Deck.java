import java.util.*;

public class Deck {
    private static Deck instance;
    private Stack<Card> deck;

    private Deck() {
        deck = new Stack<>();
        deck.addAll(Constants.allCards());
    }

    public static Deck getInstance() {
        if (instance == null) {
            instance = new Deck();
        }
        return instance;
    }

    public int size() {
        return deck.size();
    }

    public boolean isEmpty() {
        return deck.isEmpty();
    }

    public void shuffle() {
        Collections.shuffle(deck);
    }

    public Card pop() {
        if (deck.isEmpty()) {
            System.out.println("deck is empty");
            return null;
        }
        return deck.pop();
    }
}
