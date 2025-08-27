package com.st.host;

import java.util.*;

public class GameState {
    private final Set<Card> hostHand;
    private final Set<Card> clientHand;
    private final Wall[] walls;
    private final int deckSize;
    private final Map<CardColor, List<Card>> discard;
    private final boolean isClientTurn;
    private final int cauldronCount;
    private final boolean usedCauldron;
    private final boolean isClientAttacker;
    private final Winner winner;
    private final Card lastPlayedCard;

    public GameState(Set<Card> hostHand, Set<Card> clientHand, Wall[] walls, int deckSize, Map<CardColor, List<Card>> discard, boolean isClientTurn, int cauldronCount, boolean usedCauldron, boolean isClientAttacker, Winner winner, Card lastPlayedCard) {
        this.hostHand = hostHand;
        this.clientHand = clientHand;
        this.walls = walls;
        this.deckSize = deckSize;
        this.discard = discard;
        this.isClientTurn = isClientTurn;
        this.cauldronCount = cauldronCount;
        this.usedCauldron = usedCauldron;
        this.isClientAttacker = isClientAttacker;
        this.winner = winner;
        this.lastPlayedCard = lastPlayedCard;
    }

    public Set<Card> getHostHand() {
        return hostHand;
    }

    public Set<Card> getClientHand() {
        return clientHand;
    }

    public Wall[] getWalls() {
        return walls;
    }

    public int getDeckSize() {
        return deckSize;
    }

    public Map<CardColor, List<Card>> getDiscard() {
        return discard;
    }

    public boolean isClientTurn() {
        return isClientTurn;
    }

    public int getCauldronCount() {
        return cauldronCount;
    }

    public boolean hasUsedCauldron() {
        return usedCauldron;
    }

    public boolean isClientAttacker() {
        return isClientAttacker;
    }

    public Winner getWinner() {
        return winner;
    }

    public Card getLastPlayedCard() {
        return lastPlayedCard;
    }
}
