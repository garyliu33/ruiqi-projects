package com.st.common;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;
import java.util.TreeSet;

import com.st.proto.GameState.CardListProto;
import com.st.proto.GameState.GameStateProto;

public class GameState {
    private final Set<Card> attackerHand;
    private final Set<Card> defenderHand;
    private final Wall[] walls;
    private final int deckSize;
    private final Map<CardColor, List<Card>> discard;
    private final boolean isClientTurn;
    private final int cauldronCount;
    private final boolean usedCauldron;
    private final boolean isClientAttacker;
    private final Winner winner;
    private final Card lastPlayedCard;

    public GameState(Set<Card> attackerHand, Set<Card> defenderHand, Wall[] walls, int deckSize,
            Map<CardColor, List<Card>> discard, boolean isClientTurn, int cauldronCount,
            boolean usedCauldron, boolean isClientAttacker, Winner winner, Card lastPlayedCard) {
        this.attackerHand = attackerHand;
        this.defenderHand = defenderHand;
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

    public Set<Card> getAttackerHand() {
        return attackerHand;
    }

    public Set<Card> getDefenderHand() {
        return defenderHand;
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
        for (Card c : attackerHand) {
            builder.addAttackerHand(c.toProto());
        }
        for (Card c : defenderHand) {
            builder.addDefenderHand(c.toProto());
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

    public static GameState fromProto(GameStateProto proto) {
        Set<Card> attackerHand = new TreeSet<>();
        for (int i = 0; i < proto.getAttackerHandCount(); i++) {
            attackerHand.add(Card.fromProto(proto.getAttackerHand(i)));
        }

        Set<Card> defenderHand = new TreeSet<>();
        for (int i = 0; i < proto.getDefenderHandCount(); i++) {
            defenderHand.add(Card.fromProto(proto.getDefenderHand(i)));
        }

        Wall[] walls = new Wall[proto.getWallsCount()];
        for (int i = 0; i < walls.length; i++) {
            walls[i] = Wall.fromProto(proto.getWalls(i));
        }

        Map<CardColor, List<Card>> discard = new TreeMap<>();
        Map<Integer, CardListProto> protoMap = proto.getDiscardMap();
        for (Integer key : protoMap.keySet()) {
            List<Card> cards = new ArrayList<>();
            CardListProto cardListProto = protoMap.get(key);
            for (int i = 0; i < cardListProto.getCardListCount(); i++) {
                cards.add(Card.fromProto(cardListProto.getCardList(i)));
            }
            discard.put(CardColor.values()[key], cards);
        }

        return new GameState(attackerHand, defenderHand, walls, proto.getDeckSize(), discard,
                proto.getIsClientTurn(), proto.getCauldronCount(), proto.getUsedCauldron(),
                proto.getIsClientAttacker(), Winner.fromProto(proto.getWinner()),
                proto.hasLastPlayedCard() ? Card.fromProto(proto.getLastPlayedCard()) : null);
    }
}
