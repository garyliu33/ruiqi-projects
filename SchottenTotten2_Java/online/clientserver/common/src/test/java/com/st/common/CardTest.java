package com.st.common;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assertions.assertFalse;
import org.junit.jupiter.api.Test;

import com.st.proto.Card.CardProto;

public class CardTest {

    // Test cases for the constructor and getters
    @Test
    public void testCardCreation() {
        Card card = new Card(CardColor.RED, 10);
        assertEquals(CardColor.RED, card.getColor());
        assertEquals(10, card.getValue());
    }

    // Test cases for special cards
    @Test
    public void testSpecialCards() {
        assertEquals(CardColor.ACTION_COLOR, Card.RETREAT.getColor());
        assertEquals(-1, Card.RETREAT.getValue());
        assertEquals(CardColor.ACTION_COLOR, Card.CAULDRON.getColor());
        assertEquals(-2, Card.CAULDRON.getValue());
    }

    // Test cases for equals() method
    @Test
    public void testEquals() {
        Card card1 = new Card(CardColor.RED, 5);
        Card card2 = new Card(CardColor.RED, 5);
        Card card3 = new Card(CardColor.BLUE, 5);
        Card card4 = new Card(CardColor.RED, 6);

        // Test with same object
        assertTrue(card1.equals(card1));
        
        // Test with equal objects
        assertTrue(card1.equals(card2));
        
        // Test with different objects
        assertFalse(card1.equals(card3));
        assertFalse(card1.equals(card4));
        
        // Test with null
        assertFalse(card1.equals(null));
        
        // Test with different class type
        assertFalse(card1.equals("A string"));
    }

    // Test cases for hashCode() method
    @Test
    public void testHashCode() {
        Card card1 = new Card(CardColor.GREEN, 8);
        Card card2 = new Card(CardColor.GREEN, 8);
        Card card3 = new Card(CardColor.YELLOW, 8);

        // Equal objects must have equal hash codes
        assertEquals(card1.hashCode(), card2.hashCode());
        
        // Unequal objects may have different hash codes
        assertNotEquals(card1.hashCode(), card3.hashCode());
    }

    // Test cases for compareTo() method
    @Test
    public void testCompareTo() {
        // Compare cards with the same color
        Card red5 = new Card(CardColor.RED, 5);
        Card red10 = new Card(CardColor.RED, 10);
        assertTrue(red5.compareTo(red10) < 0);
        assertTrue(red10.compareTo(red5) > 0);
        assertTrue(red5.compareTo(new Card(CardColor.RED, 5)) == 0);
        
        // Compare cards with different colors
        Card blue5 = new Card(CardColor.BLUE, 5);
        assertTrue(red5.compareTo(blue5) < 0); // Assuming CardColor.RED is less than CardColor.BLUE
        assertTrue(blue5.compareTo(red5) > 0);
        
        // Test with special cards
        Card retreatCard = Card.RETREAT;
        Card cauldronCard = Card.CAULDRON;
        // ACTION_COLOR should be greater than other colors based on CardColor enum order
        Card yellowCard = new Card(CardColor.YELLOW, 1);
        assertTrue(retreatCard.compareTo(yellowCard) > 0);
        assertTrue(cauldronCard.compareTo(yellowCard) > 0);
        assertTrue(retreatCard.compareTo(cauldronCard) > 0); // -1 is greater than -2
    }

    // Test cases for toProto() and fromProto()
    @Test
    public void testToProtoAndFromProto() {
        Card originalCard = new Card(CardColor.GREEN, 7);
        CardProto proto = originalCard.toProto();

        assertNotNull(proto);
        assertEquals(proto.getColor(), CardColor.GREEN.toProto());
        assertEquals(proto.getValue(), 7);

        Card newCard = Card.fromProto(proto);
        assertEquals(originalCard.getColor(), newCard.getColor());
        assertEquals(originalCard.getValue(), newCard.getValue());
        assertEquals(originalCard, newCard);
    }
}