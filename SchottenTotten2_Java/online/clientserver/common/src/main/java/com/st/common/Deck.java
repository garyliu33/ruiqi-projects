package com.st.common;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Deque;
import java.util.List;

public class Deck {
    private final Deque<Card> deck;

    public Deck() {
        deck = new ArrayDeque<>();
        deck.addAll(Constants.ALL_CARDS);
    }

    public Deck(int size) {
        deck = new ArrayDeque<>(size);
        for (int i = 0; i < size; i++) {
            deck.add(null); // Placeholder for client-side representation
        }
    }

    public int size() {
        return deck.size();
    }

    public void shuffle() {
        List<Card> cards = new ArrayList<>(deck);
        Collections.shuffle(cards);
        deck.clear();
        deck.addAll(cards);
    }

    public Card pop() {
        if (deck.isEmpty()) {
            return null;
        }
        return deck.pop();
    }

    public void reset() {
        deck.clear();
        deck.addAll(Constants.ALL_CARDS);
        shuffle();
    }
}