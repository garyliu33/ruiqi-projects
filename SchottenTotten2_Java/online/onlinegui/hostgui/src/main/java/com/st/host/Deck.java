package com.st.host;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Deque;
import java.util.List;

import com.st.common.Card;
import com.st.common.Constants;

public class Deck {
    private final Deque<Card> deck;

    public Deck() {
        deck = new ArrayDeque<>();
        deck.addAll(Constants.ALL_CARDS);
    }

    public int size() {
        return deck.size();
    }

    public boolean isEmpty() {
        return deck.isEmpty();
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
