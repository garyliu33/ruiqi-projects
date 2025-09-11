package com.st.common;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.ArrayList;
import java.util.List;
import java.util.Set;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import com.st.proto.Wall.WallProto;
import com.st.common.Wall.Status;

public class WallTest {

    private Wall wall;
    private Card red5;
    private Card red6;
    private Card blue5;
    private Card blue6;
    private Card green11;
    private Card green0;

    @BeforeEach
    public void setUp() {
        wall = new Wall(0);
        red5 = new Card(CardColor.RED, 5);
        red6 = new Card(CardColor.RED, 6);
        blue5 = new Card(CardColor.BLUE, 5);
        blue6 = new Card(CardColor.BLUE, 6);
        green11 = new Card(CardColor.GREEN, 11);
        green0 = new Card(CardColor.GREEN, 0);
    }

    // --- Constructor and Getters Tests ---

    @Test
    public void testWallCreationWithDefaultStatus() {
        Wall newWall = new Wall(1);
        assertEquals(1, newWall.getWallIndex());
        assertEquals(4, newWall.getLength()); // Wall 1 has length 4
        assertEquals(WallPattern.NONE, newWall.getPattern()); // Wall 1 has pattern NONE
        assertEquals(Status.INTACT, newWall.getStatus());
        assertTrue(newWall.getAttackerCards().isEmpty());
        assertTrue(newWall.getDefenderCards().isEmpty());
    }

    @Test
    public void testWallCreationWithExplicitStatus() {
        Wall newWall = new Wall(2, Status.DAMAGED);
        assertEquals(2, newWall.getWallIndex());
        assertEquals(3, newWall.getLength()); // Wall 2 damaged length is 3
        assertEquals(WallPattern.COLOR, newWall.getPattern()); // Wall 2 damaged pattern is COLOR
        assertEquals(Status.DAMAGED, newWall.getStatus());
    }

    @Test
    public void testGetters() {
        assertEquals(0, wall.getWallIndex());
        assertEquals(3, wall.getLength()); // Wall 0 has length 3
        assertEquals(WallPattern.PLUS, wall.getPattern()); // Wall 0 has pattern PLUS
        assertEquals(Status.INTACT, wall.getStatus());
    }

    // --- Status Enum Tests ---

    @Test
    public void testStatusToAndFromProto() {
        assertEquals(Wall.Status.INTACT, Wall.Status.fromProto(Status.INTACT.toProto()));
        assertEquals(Wall.Status.DAMAGED, Wall.Status.fromProto(Status.DAMAGED.toProto()));
        assertEquals(Wall.Status.BROKEN, Wall.Status.fromProto(Status.BROKEN.toProto()));
    }

    // --- Card Management Tests ---

    @Test
    public void testContains() {
        wall.playCard(red5, true);
        wall.playCard(blue6, false);
        assertTrue(wall.contains(red5));
        assertTrue(wall.contains(blue6));
        assertFalse(wall.contains(red6));
    }

    @Test
    public void testReset() {
        wall.playCard(red5, true);
        wall.playCard(blue6, false);
        assertEquals(1, wall.getAttackerCards().size());
        assertEquals(1, wall.getDefenderCards().size());

        wall.damage(); // Make it damaged
        
        assertEquals(Status.DAMAGED, wall.getStatus());

        wall.reset();
        assertEquals(Status.INTACT, wall.getStatus());
        assertTrue(wall.getAttackerCards().isEmpty());
        assertTrue(wall.getDefenderCards().isEmpty());
    }

    // --- PlayCard Method Tests ---

    @Test
    public void testPlayRegularCardSuccess() {
        PlayResult result = wall.playCard(red5, true);
        assertEquals(PlayResult.Type.SUCCESS, result.getResultType());
        assertEquals(1, wall.getAttackerCards().size());
        assertEquals(0, result.getToDiscard().size());
    }
    
    @Test
    public void testPlayCardWhenWallIsFull() {
        wall.playCard(red5, true);
        wall.playCard(red6, true);
        wall.playCard(blue5, true);

        PlayResult result = wall.playCard(blue6, true);
        assertEquals(PlayResult.Type.FAILURE, result.getResultType());
        assertEquals(3, wall.getAttackerCards().size());
    }

    @Test
    public void testPlayRetreatCard() {
        wall.playCard(red5, true);
        wall.playCard(red6, true);

        PlayResult result = wall.playCard(Card.RETREAT, true);
        assertEquals(PlayResult.Type.ACTION, result.getResultType());
        assertEquals(2, result.getToDiscard().size());
        assertTrue(result.getToDiscard().contains(red5));
        assertTrue(result.getToDiscard().contains(red6));
        assertTrue(wall.getAttackerCards().isEmpty());
    }
    
    @Test
    public void testPlayCauldronCardWhenAttackerHasCards() {
        wall.playCard(red5, true);
        
        PlayResult result = wall.playCard(Card.CAULDRON, true);
        assertEquals(PlayResult.Type.ACTION, result.getResultType());
        assertEquals(1, result.getToDiscard().size());
        assertTrue(result.getToDiscard().contains(red5));
        assertTrue(wall.getAttackerCards().isEmpty());
    }

    @Test
    public void testPlayCauldronCardWhenAttackerHasNoCards() {
        PlayResult result = wall.playCard(Card.CAULDRON, true);
        assertEquals(PlayResult.Type.FAILURE, result.getResultType());
        assertTrue(wall.getAttackerCards().isEmpty());
    }
    
    @Test
    public void testPlaySpecialValueCardWithCounterpart() {
        wall.playCard(green11, false); // Defender plays a 11
        PlayResult result = wall.playCard(green0, true); // Attacker plays a 0
        
        assertEquals(PlayResult.Type.SUCCESS, result.getResultType());
        assertEquals(2, result.getToDiscard().size());
        assertTrue(result.getToDiscard().contains(green11));
        assertTrue(result.getToDiscard().contains(green0));
        assertTrue(wall.getAttackerCards().isEmpty());
        assertTrue(wall.getDefenderCards().isEmpty());
    }

    // --- DeclareControl Method Tests ---

    @Test
    public void testDeclareControlAttackerWin() {
        // Attacker wins with a higher strength
        Wall highStrengthWall = new Wall(2); // Wall 2 has length 3
        highStrengthWall.playCard(new Card(CardColor.RED, 5), true);
        highStrengthWall.playCard(new Card(CardColor.RED, 6), true);
        highStrengthWall.playCard(new Card(CardColor.RED, 7), true); // Attacker formation is now complete

        List<Card> defenderCards = new ArrayList<>();
        defenderCards.add(new Card(CardColor.BLUE, 1));
        defenderCards.add(new Card(CardColor.BLUE, 2));
        defenderCards.add(new Card(CardColor.BLUE, 3));
        Set<Card> discarded = highStrengthWall.declareControl(defenderCards);

        assertEquals(Status.DAMAGED, highStrengthWall.getStatus());
        assertEquals(3, discarded.size());
    }
    
    @Test
    public void testDeclareControlAttackerFinishedFirstWin() {
        // Attacker wins with equal strength but finished first
        Wall equalStrengthWall = new Wall(3); // Wall 3 has length 2
        equalStrengthWall.playCard(new Card(CardColor.RED, 5), true);
        equalStrengthWall.playCard(new Card(CardColor.RED, 6), true);
        // Play one card for defender to trigger attackerFinishedFirst
        equalStrengthWall.playCard(new Card(CardColor.BLUE, 5), false); 

        List<Card> list = new ArrayList<>();
        list.add(new Card(CardColor.BLUE, 4));
        Set<Card> discarded = equalStrengthWall.declareControl(list);

        assertEquals(Status.DAMAGED, equalStrengthWall.getStatus());
        assertEquals(3, discarded.size());
    }

    @Test
    public void testDeclareControlDefenderWin() {
        // Defender wins with higher strength
        Wall defenderWinWall = new Wall(3); // Wall 3 has length 2
        defenderWinWall.playCard(new Card(CardColor.RED, 1), true);
        defenderWinWall.playCard(new Card(CardColor.RED, 2), true);
        
        List<Card> defenderCards = new ArrayList<>();
        defenderCards.add(new Card(CardColor.BLUE, 10));
        defenderCards.add(new Card(CardColor.BLUE, 11));
        Set<Card> discarded = defenderWinWall.declareControl(defenderCards);
        
        assertEquals(Status.INTACT, defenderWinWall.getStatus());
        assertTrue(discarded.isEmpty());
    }

    // --- Damage Method Tests ---
    
    @Test
    public void testDamageFromIntactToDamaged() {
        wall.playCard(red5, true);
        wall.playCard(blue6, false);
        Set<Card> discarded = wall.damage();
        
        assertEquals(Status.DAMAGED, wall.getStatus());
        assertEquals(2, discarded.size());
        assertTrue(wall.getAttackerCards().isEmpty());
        assertTrue(wall.getDefenderCards().isEmpty());
    }
    
    @Test
    public void testDamageFromDamagedToBroken() {
        // Set the wall to a damaged state first
        Wall damagedWall = new Wall(0, Status.DAMAGED);
        damagedWall.playCard(red5, true);
        damagedWall.playCard(blue6, false);
        
        Set<Card> discarded = damagedWall.damage();
        
        assertEquals(Status.BROKEN, damagedWall.getStatus());
        assertEquals(2, discarded.size());
        assertTrue(damagedWall.getAttackerCards().isEmpty());
        assertTrue(damagedWall.getDefenderCards().isEmpty());
    }

    // --- Protobuf Conversion Tests ---

    @Test
    public void testToProtoAndFromProtoRoundTrip() {
        wall.playCard(red5, true);
        wall.playCard(blue6, false);
        wall.damage(); // Set it to a damaged state

        WallProto proto = wall.toProto();
        assertNotNull(proto);

        Wall newWall = Wall.fromProto(proto);

        assertEquals(wall.getWallIndex(), newWall.getWallIndex());
        assertEquals(wall.getStatus(), newWall.getStatus());
        // After damage, cards are cleared.
        assertEquals(0, newWall.getAttackerCards().size());
        assertEquals(0, newWall.getDefenderCards().size());
        assertTrue(wall.getAttackerCards().isEmpty());
        assertTrue(wall.getDefenderCards().isEmpty());
    }
}