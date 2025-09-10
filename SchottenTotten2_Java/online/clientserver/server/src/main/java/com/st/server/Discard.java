package com.st.server;

import com.st.common.Card;
import com.st.common.CardColor;

import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;
import java.util.TreeSet;

public class Discard {
    private final Set<Card> discard;

    public Discard() {
        discard = new TreeSet<>();
    }

    public void addAll(Collection<Card> cards) {
        discard.addAll(cards);
    }

    public boolean contains(Card card) {
        return discard.contains(card);
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