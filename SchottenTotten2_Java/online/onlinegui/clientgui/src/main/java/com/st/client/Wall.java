package com.st.client;

import java.util.ArrayList;
import java.util.List;

import com.st.proto.Wall.StatusProto;
import com.st.proto.Wall.WallProto;

public class Wall {
    private final int wallIndex;
    private final Status status;
    private final int length;
    private final WallPattern pattern;

    private final List<Card> attackerCards;
    private final List<Card> defenderCards;

    public enum Status {
        BROKEN, DAMAGED, INTACT;

        public static Status fromProto(StatusProto proto) {
            switch (proto) {
                case StatusProto.BROKEN -> {
                    return Status.BROKEN;
                }
                case StatusProto.DAMAGED -> {
                    return Status.DAMAGED;
                }
                case StatusProto.INTACT -> {
                    return Status.INTACT;
                }
            }
            throw new AssertionError();
        }
    }

    public Wall(int wallIndex, int length, WallPattern pattern, Status status) {
        this.wallIndex = wallIndex;
        this.status = status;
        this.length = length;
        this.pattern = pattern;

        this.attackerCards = new ArrayList<>();
        this.defenderCards = new ArrayList<>();
    }

    public List<Card> getAttackerCards() {
        return attackerCards;
    }

    public List<Card> getDefenderCards() {
        return defenderCards;
    }

    public Status getStatus() {
        return status;
    }

    public WallPattern getPattern() {
        return pattern;
    }

    public int getLength() {
        return length;
    }

    public int getWallIndex() {
        return wallIndex;
    }

    public static Wall fromProto(WallProto proto) {
        Wall wall = new Wall(proto.getWallIndex(), proto.getLength(),
                WallPattern.fromProto(proto.getPattern()),
                Status.fromProto(proto.getStatus()));
        for (int i = 0; i < proto.getAttackerCardsCount(); i++) {
            wall.attackerCards.add(Card.fromProto(proto.getAttackerCards(i)));
        }
        for (int i = 0; i < proto.getDefenderCardsCount(); i++) {
            wall.defenderCards.add(Card.fromProto(proto.getDefenderCards(i)));
        }
        return wall;
    }
}
