package com.st.server;

import com.st.common.Card;

import java.util.Set;
import java.util.TreeSet;

public class Hand {
    private final Set<Card> cards;

    public Hand() {
        cards = new TreeSet<>();
    }

    public Set<Card> getCards() {
        return cards;
    }

    public void remove(Card card) {
        cards.remove(card);
    }
}