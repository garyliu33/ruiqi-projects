package com.st.common;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import org.junit.jupiter.api.Test;

import com.st.proto.Card.ColorProto;
import java.awt.Color;

public class CardColorTest {

    // Test cases for getDisplayColor()
    @Test
    public void testGetDisplayColor() {
        assertEquals(new Color(200, 46, 46), CardColor.RED.getDisplayColor());
        assertEquals(new Color(61, 165, 209), CardColor.BLUE.getDisplayColor());
        assertEquals(new Color(222, 195, 76), CardColor.YELLOW.getDisplayColor());
        assertEquals(new Color(63, 175, 55), CardColor.GREEN.getDisplayColor());
        assertEquals(new Color(110, 110, 110), CardColor.GRAY.getDisplayColor());
        assertEquals(Color.BLACK, CardColor.ACTION_COLOR.getDisplayColor());
    }

    // Test cases for getAllColors()
    @Test
    public void testGetAllColors() {
        CardColor[] allColors = CardColor.getAllColors();
        CardColor[] expectedColors = {
            CardColor.RED, CardColor.BLUE, CardColor.YELLOW, CardColor.GREEN, CardColor.GRAY
        };

        assertEquals(expectedColors.length, allColors.length);
        assertArrayEquals(expectedColors, allColors);
    }
    
    // Test cases for toProto()
    @Test
    public void testToProto() {
        assertEquals(ColorProto.RED, CardColor.RED.toProto());
        assertEquals(ColorProto.BLUE, CardColor.BLUE.toProto());
        assertEquals(ColorProto.YELLOW, CardColor.YELLOW.toProto());
        assertEquals(ColorProto.GREEN, CardColor.GREEN.toProto());
        assertEquals(ColorProto.GRAY, CardColor.GRAY.toProto());
        assertEquals(ColorProto.ACTION, CardColor.ACTION_COLOR.toProto());
    }

    // Test cases for fromProto()
    @Test
    public void testFromProto() {
        assertEquals(CardColor.RED, CardColor.fromProto(ColorProto.RED));
        assertEquals(CardColor.BLUE, CardColor.fromProto(ColorProto.BLUE));
        assertEquals(CardColor.YELLOW, CardColor.fromProto(ColorProto.YELLOW));
        assertEquals(CardColor.GREEN, CardColor.fromProto(ColorProto.GREEN));
        assertEquals(CardColor.GRAY, CardColor.fromProto(ColorProto.GRAY));
        assertEquals(CardColor.ACTION_COLOR, CardColor.fromProto(ColorProto.ACTION));
    }
}