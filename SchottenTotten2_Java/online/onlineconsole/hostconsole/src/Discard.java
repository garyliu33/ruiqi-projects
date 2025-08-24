import java.util.*;
import java.io.*;
import java.net.*;

public class Discard {
    private static Discard instance;
    private final Set<Card> discard;

    private Discard() {
        discard = new TreeSet<>();
    }

    public static synchronized Discard getInstance() {
        if (instance == null) {
            instance = new Discard();
        }
        return instance;
    }

    public synchronized void add(Card card) {
        discard.add(card);
    }

    public synchronized void addAll(Collection<Card> cards) {
        discard.addAll(cards);
    }

    public synchronized boolean contains(Card card) {
        return discard.contains(card);
    }

    public synchronized String toString() {
        StringBuilder str = new StringBuilder();
        Card previous = null;
        for (Card card : discard) {
            str.append(card).append(" ");
            if (previous != null && !(previous.getColor() == card.getColor())) {
                str.append("\n");
            }
            previous = card;
        }
        str.append("\n");
        return str.toString();
    }

    public synchronized boolean isEmpty() {
        return discard.isEmpty();
    }

    public synchronized void clear() {
        discard.clear();
    }
}
