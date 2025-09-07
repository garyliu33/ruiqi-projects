package com.st.common;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import org.junit.jupiter.api.Test;

import com.st.common.WallPattern;
import com.st.proto.Wall.WallPatternProto;

public class WallPatternTest {

    // Test cases for getSymbol()
    @Test
    public void testGetSymbol() {
        assertEquals("C", WallPattern.COLOR.getSymbol());
        assertEquals("R", WallPattern.RUN.getSymbol());
        assertEquals("=", WallPattern.EQUALS.getSymbol());
        assertEquals("+", WallPattern.PLUS.getSymbol());
        assertEquals("-", WallPattern.MINUS.getSymbol());
        assertEquals(" ", WallPattern.NONE.getSymbol());
    }

    // Test cases for toProto()
    @Test
    public void testToProto() {
        assertEquals(WallPatternProto.COLOR, WallPattern.COLOR.toProto());
        assertEquals(WallPatternProto.RUN, WallPattern.RUN.toProto());
        assertEquals(WallPatternProto.EQUALS, WallPattern.EQUALS.toProto());
        assertEquals(WallPatternProto.PLUS, WallPattern.PLUS.toProto());
        assertEquals(WallPatternProto.MINUS, WallPattern.MINUS.toProto());
        assertEquals(WallPatternProto.NONE_PATTERN, WallPattern.NONE.toProto());
    }

    // Test cases for fromProto()
    @Test
    public void testFromProto() {
        assertEquals(WallPattern.COLOR, WallPattern.fromProto(WallPatternProto.COLOR));
        assertEquals(WallPattern.RUN, WallPattern.fromProto(WallPatternProto.RUN));
        assertEquals(WallPattern.EQUALS, WallPattern.fromProto(WallPatternProto.EQUALS));
        assertEquals(WallPattern.PLUS, WallPattern.fromProto(WallPatternProto.PLUS));
        assertEquals(WallPattern.MINUS, WallPattern.fromProto(WallPatternProto.MINUS));
        assertEquals(WallPattern.NONE, WallPattern.fromProto(WallPatternProto.NONE_PATTERN));
    }

    // Test case for a round trip conversion (toProto then fromProto)
    @Test
    public void testToAndFromProtoRoundTrip() {
        for (WallPattern pattern : WallPattern.values()) {
            WallPatternProto proto = pattern.toProto();
            assertNotNull(proto);
            WallPattern newPattern = WallPattern.fromProto(proto);
            assertEquals(pattern, newPattern);
        }
    }
}