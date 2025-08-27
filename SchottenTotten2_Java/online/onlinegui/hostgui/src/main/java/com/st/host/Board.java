package com.st.host;

import java.util.*;

public class Board {
    private final Wall[] walls;

    public Board() {
        walls = new Wall[Constants.NUM_WALLS];
        for (int i = 0; i < Constants.NUM_WALLS; i++) {
            walls[i] = new Wall(i, Constants.WALL_LENGTHS[i], Constants.DAMAGED_WALL_LENGTHS[i], Constants.WALL_PATTERNS[i], Constants.DAMAGED_WALL_PATTERNS[i]);
        }
    }

    public void clear() {
        for (Wall wall : walls) {
            wall.reset();
        }
    }

    public Wall[] getWalls() {
        return walls;
    }

    public boolean contains(Card card) {
        for (Wall wall : walls) {
            if (wall.contains(card)) {
                return true;
            }
        }
        return false;
    }

    public Set<Card> declareControl(List<Card> remainingCards) {
        Set<Card> toDiscard = new TreeSet<>();
        for (Wall wall : walls) {
            toDiscard.addAll(wall.declareControl(remainingCards));
        }
        return toDiscard;
    }

    public boolean defenderSideFull() {
        for (Wall wall : walls) {
            if (wall.getDefenderCards().size() < wall.getLength()) {
                return false;
            }
        }
        return true;
    }

    public PlayResult playCard(int wallIndex, Card card, boolean isAttacker) {
        return walls[wallIndex].playCard(card, isAttacker);
    }
}
