package com.st.host;

import java.util.List;
import java.util.Map;
import java.util.Set;

import com.st.proto.GameState.CardListProto;
import com.st.proto.GameState.GameStateProto;

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

    public GameState(Set<Card> hostHand, Set<Card> clientHand, Wall[] walls, int deckSize,
            Map<CardColor, List<Card>> discard, boolean isClientTurn, int cauldronCount,
            boolean usedCauldron, boolean isClientAttacker, Winner winner, Card lastPlayedCard) {
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

    public GameStateProto toProto() {
        GameStateProto.Builder builder = GameStateProto.newBuilder();
        for (Card c : hostHand) {
            builder.addHostHand(c.toProto());
        }
        for (Card c : clientHand) {
            builder.addClientHand(c.toProto());
        }
        for (Wall w : walls) {
            builder.addWalls(w.toProto());
        }
        builder.setDeckSize(deckSize);
        for (CardColor c : discard.keySet()) {
            List<Card> cards = discard.get(c);
            CardListProto.Builder cardListProtoBuilder = CardListProto.newBuilder();
            for (Card card : cards) {
                cardListProtoBuilder.addCardList(card.toProto());
            }
            builder.putDiscard(c.ordinal(), cardListProtoBuilder.build());
        }
        builder.setIsClientTurn(isClientTurn);
        builder.setCauldronCount(cauldronCount);
        builder.setUsedCauldron(usedCauldron);
        builder.setIsClientAttacker(isClientAttacker);
        builder.setWinner(winner.toProto());
        if (lastPlayedCard != null) {
            builder.setLastPlayedCard(lastPlayedCard.toProto());
        }
        return builder.build();
    }
}
