package com.st.host;

import java.util.*;

public class Wall {
    private final int wallIndex;
    private Status status;
    private int length;
    private final int intactLength;
    private final int damagedLength;
    private WallPattern pattern;
    private final WallPattern intactPattern;
    private final WallPattern damagedPattern;

    private final List<Card> attackerCards;
    private final List<Card> defenderCards;

    private boolean attackerFinishedFirst;
    private static final int MULTIPLIER = 100;

    public enum Status {
        BROKEN, DAMAGED, INTACT
    }

    public Wall(int wallIndex, int intactLength, int damagedLength, WallPattern intactPattern, WallPattern damagedPattern) {
        this.wallIndex = wallIndex;
        this.status = Status.INTACT;
        this.intactLength = intactLength;
        this.damagedLength = damagedLength;
        this.intactPattern = intactPattern;
        this.damagedPattern = damagedPattern;
        this.length = intactLength;
        this.pattern = intactPattern;

        this.attackerCards = new ArrayList<>();
        this.defenderCards = new ArrayList<>();
    }

    public boolean contains(Card card) {
        return attackerCards.contains(card) || defenderCards.contains(card);
    }

    public void reset() {
        attackerCards.clear();
        defenderCards.clear();
        status = Status.INTACT;
        length = intactLength;
        pattern = intactPattern;
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

    public int getIndex() {
        return wallIndex;
    }

    public PlayResult playCard(Card card, boolean isAttacker) {
        if (card.equals(Card.RETREAT)) {
            List<Card> toDiscard = new ArrayList<>(attackerCards);
            attackerCards.clear();
            return new PlayResult(PlayResult.Type.ACTION, toDiscard);
        }

        if (card.equals(Card.CAULDRON)) {
            if (!attackerCards.isEmpty()) {
                List<Card> toDiscard = List.of(attackerCards.removeLast());
                attackerFinishedFirst = false;
                return new PlayResult(PlayResult.Type.ACTION, toDiscard);
            }
            return new PlayResult(PlayResult.Type.FAILURE);
        }

        List<Card> playingSide;
        List<Card> otherSide;
        if (isAttacker) {
            playingSide = attackerCards;
            otherSide = defenderCards;
        } else {
            playingSide = defenderCards;
            otherSide = attackerCards;
        }

        if (playingSide.size() == length) {
            return new PlayResult(PlayResult.Type.FAILURE);
        }

        playingSide.add(card);
        List<Card> toDiscard = new ArrayList<>();
        int value = card.getValue();
        if (value == 0 || value == 11) {
            Card temp = new Card(card.getColor(), 11 - value);
            if (otherSide.contains(temp)) {
                playingSide.remove(card);
                otherSide.remove(temp);
                toDiscard.add(card);
                toDiscard.add(temp);
            }
        }

        if (isAttacker) {
            attackerFinishedFirst = attackerCards.size() == length && defenderCards.size() < length;
        }
        return new PlayResult(PlayResult.Type.SUCCESS, toDiscard);
    }

    public Set<Card> declareControl(List<Card> remainingCards) {
        if (attackerCards.size() == length) {
            int defenderStrength = getStrongestDefenderFormationStrength(defenderCards, remainingCards, length, Integer.MIN_VALUE);
            int attackerStrength = getStrength(attackerCards);
            if ((attackerStrength > defenderStrength) || (attackerFinishedFirst && attackerStrength >= defenderStrength)) {
                return damage();
            }
        }
        return new TreeSet<>();
    }

    private Set<Card> damage() {
        if (status == Status.DAMAGED) {
            status = Status.BROKEN;
        } else {
            status = Status.DAMAGED;
            length = damagedLength;
            pattern = damagedPattern;
        }
        Set<Card> toDiscard = new TreeSet<>(attackerCards);
        toDiscard.addAll(defenderCards);
        attackerCards.clear();
        defenderCards.clear();
        return toDiscard;
    }

    private int getStrongestDefenderFormationStrength(List<Card> currentFormation, List<Card> remainingCards, int length, int maxStrength) {
        if (currentFormation.size() == length) {
            return Math.max(getStrength(currentFormation), maxStrength);
        }
        for (int i = 0; i < remainingCards.size(); i++) {
            Card card = remainingCards.remove(i);
            currentFormation.add(card);
            maxStrength = Math.max(getStrongestDefenderFormationStrength(currentFormation, remainingCards, length, maxStrength), maxStrength);
            currentFormation.remove(card);
            remainingCards.add(i, card);
        }
        return maxStrength;
    }

    private int getStrength(List<Card> formation) {
        int sum = 0;
        for (Card card : formation) {
            sum += card.getValue();
        }
        FormationType type = getFormationType(formation);

        switch (pattern) {
            case WallPattern.PLUS -> {
                type = FormationType.SUM;
            }
            case WallPattern.MINUS -> {
                type = FormationType.SUM;
                sum *= -1;
            }
            case WallPattern.COLOR -> {
                if (type == FormationType.SAME_STRENGTH || type == FormationType.RUN) {
                    type = FormationType.SUM;
                }
            }
            case WallPattern.RUN -> {
                if (type == FormationType.SAME_STRENGTH || type == FormationType.COLOR) {
                    type = FormationType.SUM;
                }
            }
            case WallPattern.EQUALS -> {
                if (type == FormationType.COLOR_RUN || type == FormationType.COLOR || type == FormationType.RUN) {
                    type = FormationType.SUM;
                }
            }
        }
        return type.getStrength() * MULTIPLIER + sum;
    }

    private FormationType getFormationType(List<Card> formation) {
        Set<CardColor> colors = new TreeSet<>();
        List<Integer> values = new ArrayList<>();
        for (Card card : formation) {
            colors.add(card.getColor());
            values.add(card.getValue());
        }
        Collections.sort(values);

        Set<Integer> diffs = new HashSet<>();
        for (int i = 0; i < formation.size() - 1; i++) {
            diffs.add(values.get(i + 1) - values.get(i));
        }

        if (colors.size() == 1) {
            return diffs.size() == 1 && diffs.contains(1) ? FormationType.COLOR_RUN : FormationType.COLOR;
        }

        if (diffs.size() == 1) {
            if (diffs.contains(0)) {
                return FormationType.SAME_STRENGTH;
            }
            if (diffs.contains(1)) {
                return FormationType.RUN;
            }
        }

        return FormationType.SUM;
    }
}
