package com.st.host;

import com.st.proto.ClientMove.ClientMoveProto;

public record ClientMove(Card card, int wallIndex) {
    public static ClientMove fromProto(ClientMoveProto proto) {
        return new ClientMove(Card.fromProto(proto.getCard()), proto.getWallIndex());
    }
}