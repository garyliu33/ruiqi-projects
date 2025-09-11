package com.st.common;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import org.junit.jupiter.api.Test;

import com.st.proto.Card.CardProto;
import com.st.proto.ClientMove.ClientMoveProto;

public class ClientMoveTest {

    // Test case for the constructor and accessors
    @Test
    public void testClientMoveCreation() {
        Card card = new Card(CardColor.BLUE, 5);
        int wallIndex = 2;
        ClientMove clientMove = new ClientMove(card, wallIndex);

        assertEquals(card, clientMove.card());
        assertEquals(wallIndex, clientMove.wallIndex());
    }

    // Test case for toProto() conversion
    @Test
    public void testToProto() {
        Card card = new Card(CardColor.GREEN, 8);
        int wallIndex = 1;
        ClientMove clientMove = new ClientMove(card, wallIndex);

        ClientMoveProto proto = clientMove.toProto();

        assertNotNull(proto);
        assertEquals(card.getValue(), proto.getCard().getValue());
        assertEquals(card.getColor().toProto(), proto.getCard().getColor());
        assertEquals(wallIndex, proto.getWallIndex());
    }

    // Test case for fromProto() conversion
    @Test
    public void testFromProto() {
        CardProto cardProto = CardProto.newBuilder()
                .setColor(CardColor.YELLOW.toProto())
                .setValue(12)
                .build();
        int wallIndex = 3;

        ClientMoveProto clientMoveProto = ClientMoveProto.newBuilder()
                .setCard(cardProto)
                .setWallIndex(wallIndex)
                .build();

        ClientMove clientMove = ClientMove.fromProto(clientMoveProto);

        assertNotNull(clientMove);
        assertEquals(CardColor.YELLOW, clientMove.card().getColor());
        assertEquals(12, clientMove.card().getValue());
        assertEquals(wallIndex, clientMove.wallIndex());
    }
    
    // Test case for a round trip conversion (toProto then fromProto)
    @Test
    public void testToAndFromProtoRoundTrip() {
        Card originalCard = new Card(CardColor.RED, 1);
        int originalWallIndex = 0;
        ClientMove originalClientMove = new ClientMove(originalCard, originalWallIndex);

        ClientMoveProto proto = originalClientMove.toProto();
        ClientMove newClientMove = ClientMove.fromProto(proto);

        assertEquals(originalClientMove.card().getColor(), newClientMove.card().getColor());
        assertEquals(originalClientMove.card().getValue(), newClientMove.card().getValue());
        assertEquals(originalClientMove.wallIndex(), newClientMove.wallIndex());
    }
}