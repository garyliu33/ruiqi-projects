package com.st.client;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;
import java.util.TreeSet;

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

    public boolean getHasUsedCauldron() {
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

    public static GameState fromProto(GameStateProto proto) {
        Set<Card> hostHand = new TreeSet<>();
        for (int i = 0; i < proto.getHostHandCount(); i++) {
            hostHand.add(Card.fromProto(proto.getHostHand(i)));
        }

        Set<Card> clientHand = new TreeSet<>();
        for (int i = 0; i < proto.getClientHandCount(); i++) {
            clientHand.add(Card.fromProto(proto.getClientHand(i)));
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

        return new GameState(hostHand, clientHand, walls, proto.getDeckSize(), discard,
                proto.getIsClientTurn(), proto.getCauldronCount(), proto.getUsedCauldron(),
                proto.getIsClientAttacker(), Winner.fromProto(proto.getWinner()),
                proto.hasLastPlayedCard() ? Card.fromProto(proto.getLastPlayedCard()) : null);
    }
}
