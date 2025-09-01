package com.st.client;

import com.st.proto.ClientMove.ClientMoveProto;

public record ClientMove(Card card, int wallIndex) {
    public ClientMoveProto toProto() {
        ClientMoveProto.Builder builder = ClientMoveProto.newBuilder();
        return builder.setCard(card.toProto()).setWallIndex(wallIndex).build();
    }
}
