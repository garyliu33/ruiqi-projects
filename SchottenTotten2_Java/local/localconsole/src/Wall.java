import java.util.*;

public class Wall {
    private int status;
    private final int BROKEN = 2;
    private final int DAMAGED = 1;
    private final int INTACT = 0;
    private int length;
    private final int damagedLength;
    private String pattern;
    private final String damagedPattern;

    private final List<Card> attackerCards;
    private final List<Card> defenderCards;

    private boolean attackerFinishedFirst;
    private final int wallNum;

    public Wall(int length, int damagedLength, String pattern, String damagedPattern, int wallNum) {
        status = INTACT;
        this.length = length;
        this.damagedLength = damagedLength;
        this.pattern = pattern;
        this.damagedPattern = damagedPattern;

        attackerCards = new ArrayList<>();
        defenderCards = new ArrayList<>();

        attackerFinishedFirst = false;
        this.wallNum = wallNum;
    }

    public Set<Card> damage() {
        if (status == DAMAGED) {
            status = BROKEN;
            return new TreeSet<>();
        } else {
            status = DAMAGED;
            Set<Card> toDiscard = new TreeSet<>();
            toDiscard.addAll(attackerCards);
            toDiscard.addAll(defenderCards);
            attackerCards.clear();
            defenderCards.clear();
            length = damagedLength;
            pattern = damagedPattern;
            return toDiscard;
        }
    }

    public boolean isDamaged() {
        return status == DAMAGED;
    }

    public boolean isBroken() {
        return status == BROKEN;
    }

    public List<Card> retreat() {
        List<Card> toDiscard = List.copyOf(attackerCards);
        attackerCards.clear();
        return toDiscard;
    }

    public Card cauldron() {
        if (attackerCards.isEmpty()) {
            return null;
        }
        return attackerCards.removeLast();
    }

    public boolean declareControl(List<Card> remainingCards) {
        if (attackerCards.size() == length) {
            int defenderStrength = getStrongestDefenderFormationStrength(defenderCards, remainingCards, length, Integer.MIN_VALUE);
            int attackerStrength = getStrength(attackerCards);
            if (attackerFinishedFirst) {
                return attackerStrength >= defenderStrength;
            } else {
                return attackerStrength > defenderStrength;
            }
        }
        return false;
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

    public int playCard(Card card, boolean attacker) {
        List<Card> playingSide;
        List<Card> otherSide;
        if (attacker) {
            playingSide = attackerCards;
            otherSide = defenderCards;
        } else {
            playingSide = defenderCards;
            otherSide = attackerCards;
        }

        if (playingSide.size() == length) {
            System.out.println("no more space");
            return -1;
        }

        int value = card.getValue();
        if (value == 0 || value == 11) {
            Card temp = new Card(card.getColor(), 11 - value);
            if (otherSide.contains(temp)) {
                playingSide.remove(card);
                otherSide.remove(temp);
                return Constants.colors.indexOf(card.getColor()) + 1;
            }
        }
        playingSide.add(card);
        attackerFinishedFirst = attackerCards.size() == length && defenderHasSpace();
        return 0;
    }

    public void display() {
        for (int i = Constants.longestWall() - 1; i >= 0; i--) {
            if (i >= attackerCards.size()) {
                System.out.print(Constants.cardSpace() + " ");
            } else {
                System.out.print(attackerCards.get(i).toString() + " ");
            }
        }
        System.out.print(Constants.leftWalls[status] + wallNum + Constants.rightWalls[status] + " ");
        System.out.print("  ".repeat(Constants.longestWall() - length));
        for (int i = 0; i < length; i++) {
            System.out.print("[" + pattern + "] ");
        }
        System.out.print("  ".repeat(Constants.longestWall() - length));
        System.out.print(Constants.leftWalls[status] + wallNum + Constants.rightWalls[status]);

        for (Card card : defenderCards) {
            System.out.print(" " + card.toString());
        }
        System.out.println();
    }

    public boolean contains(Card card) {
        return attackerCards.contains(card) || defenderCards.contains(card);
    }

    public boolean defenderHasSpace() {
        return defenderCards.size() < length;
    }

    private int getStrength(List<Card> formation) {
        int sum = 0;
        for (Card card : formation) {
            sum += card.getValue();
        }
        int type = getPatternType(formation);

        switch (pattern) {
            case Constants.PLUS -> {
                return sum;
            }
            case Constants.MINUS -> {
                return -sum;
            }
            case Constants.COLOR -> {
                if (type == 3 || type == 1) {
                    return sum;
                } else {
                    return type * 100 + sum;
                }
            }
            case Constants.RUN -> {
                if (type == 3 || type == 2) {
                    return sum;
                } else {
                    return type * 100 + sum;
                }
            }
            case Constants.EQUALS -> {
                if (type == 4 || type == 2 || type == 1) {
                    return sum;
                } else {
                    return type * 100 + sum;
                }
            }
            default -> {
                return type * 100 + sum;
            }
        }
    }

    private int getPatternType(List<Card> formation) {
        Set<String> colors = new TreeSet<>();
        List<Integer> values = new ArrayList<>();
        for (Card card : formation) {
            colors.add(card.getColor());
            values.add(card.getValue());
        }
        Collections.sort(values);

        boolean allSame = true;
        int first = values.getFirst();
        for (int i = 1; i < values.size(); i++) {
            if (values.get(i) != first) {
                allSame = false;
                break;
            }
        }
        if (allSame) {
            return 3;
        }

        Set<Integer> diffs = new TreeSet<>();
        for (int i = 0; i < formation.size() - 1; i++) {
            diffs.add(values.get(i + 1) - values.get(i));
        }

        if (colors.size() == 1) {
            if (diffs.size() == 1 && diffs.contains(1)) {
                return 4;
            } else {
                return 2;
            }
        } else {
            if (diffs.size() == 1 && diffs.contains(1)) {
                return 1;
            }
        }

        return 0;
    }
}
