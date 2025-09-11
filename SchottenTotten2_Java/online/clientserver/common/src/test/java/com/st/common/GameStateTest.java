package com.st.common;

import static org.junit.jupiter.api.Assertions.*;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;
import java.util.TreeSet;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import com.st.proto.GameState.CardListProto;
import com.st.proto.GameState.GameStateProto;
import com.st.proto.GameState.WinnerProto;
import com.st.proto.Card.CardProto;
import com.google.protobuf.InvalidProtocolBufferException;

class GameStateTest {

    private GameState gameState;
    private Set<Card> attackerHand;
    private Set<Card> defenderHand;
    private Wall[] walls;
    private int deckSize;
    private Map<CardColor, List<Card>> discard;
    private boolean isClientTurn;
    private int cauldronCount;
    private boolean usedCauldron;
    private boolean isClientAttacker;
    private Winner winner;
    private Card lastPlayedCard;

    @BeforeEach
    void setUp() {
        // Initialize all the fields with dummy data for testing
        attackerHand = new TreeSet<>();
        attackerHand.add(new Card(CardColor.RED, 1));
        attackerHand.add(new Card(CardColor.BLUE, 2));

        defenderHand = new TreeSet<>();
        defenderHand.add(new Card(CardColor.GREEN, 3));
        defenderHand.add(new Card(CardColor.YELLOW, 4));

        walls = new Wall[] {
            new Wall(0),
            new Wall(1)
        };

        deckSize = 25;
        discard = new TreeMap<>();
        List<Card> redDiscard = new ArrayList<>();
        redDiscard.add(new Card(CardColor.RED, 5));
        discard.put(CardColor.RED, redDiscard);

        isClientTurn = true;
        cauldronCount = 2;
        usedCauldron = false;
        isClientAttacker = false;
        winner = Winner.ATTACKER;
        lastPlayedCard = new Card(CardColor.GRAY, 10);

        gameState = new GameState(
                attackerHand, defenderHand, walls, new Deck(deckSize), discard,
                isClientTurn, cauldronCount, usedCauldron, isClientAttacker,
                winner, lastPlayedCard);
    }

    @Test
    void testGameStateConstructorAndGetters() {
        // Verify all fields are correctly initialized
        assertEquals(attackerHand, gameState.getAttackerHand());
        assertEquals(defenderHand, gameState.getDefenderHand());
        assertArrayEquals(walls, gameState.getWalls());
        assertEquals(deckSize, gameState.getDeck().size());
        assertEquals(discard, gameState.getDiscard());
        assertEquals(isClientTurn, gameState.isClientTurn());
        assertEquals(cauldronCount, gameState.getCauldronCount());
        assertEquals(usedCauldron, gameState.hasUsedCauldron());
        assertEquals(isClientAttacker, gameState.isClientAttacker());
        assertEquals(winner, gameState.getWinner());
        assertEquals(lastPlayedCard, gameState.getLastPlayedCard());
    }

    @Test
    void testToProto() {
        GameStateProto proto = gameState.toProto();

        // Verify the generated proto object fields
        assertEquals(2, proto.getAttackerHandCount());
        assertEquals(2, proto.getDefenderHandCount());
        assertEquals(2, proto.getWallsCount());
        assertEquals(deckSize, proto.getDeckSize());
        assertEquals(1, proto.getDiscardMap().size());
        assertTrue(proto.getDiscardMap().containsKey(CardColor.RED.ordinal()));
        assertEquals(1, proto.getDiscardMap().get(CardColor.RED.ordinal()).getCardListCount());
        assertEquals(isClientTurn, proto.getIsClientTurn());
        assertEquals(cauldronCount, proto.getCauldronCount());
        assertEquals(usedCauldron, proto.getUsedCauldron());
        assertEquals(isClientAttacker, proto.getIsClientAttacker());
        assertEquals(WinnerProto.ATTACKER, proto.getWinner());

        // Verify some converted cards to ensure data is correct
        CardProto firstAttackerCardProto = proto.getAttackerHandList().stream()
                .filter(c -> c.getValue() == 2)
                .findFirst().orElseThrow();
        assertEquals(CardColor.BLUE.toProto(), firstAttackerCardProto.getColor());
        assertEquals(2, firstAttackerCardProto.getValue());

        CardProto firstDiscardCardProto = proto.getDiscardMap().get(CardColor.RED.ordinal()).getCardList(0);
        assertEquals(CardColor.RED.toProto(), firstDiscardCardProto.getColor());
        assertEquals(5, firstDiscardCardProto.getValue());
    }

    @Test
    void testFromProto() throws InvalidProtocolBufferException {
        // Create a proto object to be converted back
        GameStateProto.Builder protoBuilder = GameStateProto.newBuilder()
            .setDeckSize(20)
            .setIsClientTurn(false)
            .setCauldronCount(1)
            .setUsedCauldron(true)
            .setIsClientAttacker(true)
            .setWinner(WinnerProto.DEFENDER)
            .setLastPlayedCard(new Card(CardColor.GRAY, 10).toProto());

        protoBuilder.addAttackerHand(new Card(CardColor.RED, 1).toProto());
        protoBuilder.addDefenderHand(new Card(CardColor.BLUE, 2).toProto());
        
        // Add wall proto
        Wall wall = new Wall(0);
        wall.getAttackerCards().add(new Card(CardColor.GREEN, 1));
        wall.getDefenderCards().add(new Card(CardColor.YELLOW, 2));
        protoBuilder.addWalls(wall.toProto());

        // Add discard proto map
        CardListProto cardListProto = CardListProto.newBuilder()
                .addCardList(new Card(CardColor.YELLOW, 3).toProto())
                .build();
        protoBuilder.putDiscard(CardColor.YELLOW.ordinal(), cardListProto);
        
        GameStateProto proto = protoBuilder.build();

        GameState newGameState = GameState.fromProto(proto);
        
        // Verify that the new GameState object has the correct values
        assertEquals(20, newGameState.getDeck().size());
        assertFalse(newGameState.isClientTurn());
        assertEquals(1, newGameState.getCauldronCount());
        assertTrue(newGameState.hasUsedCauldron());
        assertTrue(newGameState.isClientAttacker());
        assertEquals(Winner.DEFENDER, newGameState.getWinner());
        assertEquals(1, newGameState.getAttackerHand().size());
        assertEquals(1, newGameState.getDefenderHand().size());
        assertEquals(1, newGameState.getWalls().length);
        assertEquals(1, newGameState.getDiscard().size());
        
        // Verify that the contents of the converted lists/sets are correct
        assertTrue(newGameState.getAttackerHand().contains(new Card(CardColor.RED, 1)));
        assertTrue(newGameState.getDefenderHand().contains(new Card(CardColor.BLUE, 2)));
        assertTrue(newGameState.getDiscard().get(CardColor.YELLOW).contains(new Card(CardColor.YELLOW, 3)));

        // Verify that wall contents are correctly restored
        assertEquals(1, newGameState.getWalls()[0].getAttackerCards().size());
        assertEquals(1, newGameState.getWalls()[0].getDefenderCards().size());
        assertTrue(newGameState.getWalls()[0].getAttackerCards().contains(new Card(CardColor.GREEN, 1)));
        assertTrue(newGameState.getWalls()[0].getDefenderCards().contains(new Card(CardColor.YELLOW, 2)));
    }
}