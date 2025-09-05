package com.st.common;

import com.st.proto.ClientMove.ClientMoveProto;

public record ClientMove(Card card, int wallIndex) {
    public ClientMoveProto toProto() {
        ClientMoveProto.Builder builder = ClientMoveProto.newBuilder();
        return builder.setCard(card.toProto()).setWallIndex(wallIndex).build();
    }

    public static ClientMove fromProto(ClientMoveProto proto) {
        return new ClientMove(Card.fromProto(proto.getCard()), proto.getWallIndex());
    }
}