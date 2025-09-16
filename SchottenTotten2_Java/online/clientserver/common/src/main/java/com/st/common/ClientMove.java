package com.st.common;

import com.st.proto.ClientMove.ClientMoveProto;

public record ClientMove(Card card, int wallIndex, Role role) {
    public ClientMove(Card card, int wallIndex) {
        this(card, wallIndex, null);
    }

    public ClientMoveProto toProto() {
        ClientMoveProto.Builder builder = ClientMoveProto.newBuilder();
        builder.setCard(card.toProto()).setWallIndex(wallIndex);
        if (role != null) {
            builder.setRole(role.toProto());
        }
        return builder.build();
    }

    public static ClientMove fromProto(ClientMoveProto proto) {
        return new ClientMove(Card.fromProto(proto.getCard()), proto.getWallIndex(), Role.fromProto(proto.getRole()));
    }
}