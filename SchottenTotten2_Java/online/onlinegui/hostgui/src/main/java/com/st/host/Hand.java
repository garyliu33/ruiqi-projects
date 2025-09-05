package com.st.host;

import java.util.Set;
import java.util.TreeSet;

import com.st.common.Card;

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
