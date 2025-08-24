import java.util.*;

public class Wall {
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
    private final int wallNum;

    private static final int MULTIPLIER = 100;

    private enum Status {
        BROKEN("  ", "  "),
        DAMAGED("| ", " |"),
        INTACT("||", "||");

        private final String leftSymbol;
        private final String rightSymbol;

        Status(String left, String right) {
            leftSymbol = left;
            rightSymbol = right;
        }

        public String getLeftSymbol() {
            return leftSymbol;
        }

        public String getRightSymbol() {
            return rightSymbol;
        }
    }

    public Wall(int intactLength, int damagedLength, WallPattern intactPattern, WallPattern damagedPattern, int wallNum) {
        status = Status.INTACT;
        this.intactLength = intactLength;
        this.damagedLength = damagedLength;
        this.intactPattern = intactPattern;
        this.damagedPattern = damagedPattern;
        length = intactLength;
        pattern = intactPattern;

        attackerCards = new ArrayList<>();
        defenderCards = new ArrayList<>();

        attackerFinishedFirst = false;
        this.wallNum = wallNum;
    }

    public int getLeftSymbolLength() {
        return status.getLeftSymbol().length();
    }

    public int getRightSymbolLength() {
        return status.getRightSymbol().length();
    }

    public Set<Card> damage() {
        if (status == Status.DAMAGED) {
            status = Status.BROKEN;
            return new TreeSet<>();
        } else {
            status = Status.DAMAGED;
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
        return status == Status.DAMAGED;
    }

    public boolean isBroken() {
        return status == Status.BROKEN;
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

    public Played playCard(Card card, boolean isAttacker) {
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
            return Played.NO_SPACE;
        }

        int value = card.getValue();
        if (value == 0 || value == 11) {
            Card temp = new Card(card.getColor(), 11 - value);
            if (otherSide.contains(temp)) {
                playingSide.remove(card);
                otherSide.remove(temp);
                Discard.getInstance().add(card);
                Discard.getInstance().add(temp);
                return Played.SUCCEEDED;
            }
        }
        playingSide.add(card);
        attackerFinishedFirst = attackerCards.size() == length && defenderHasSpace();
        return Played.SUCCEEDED;
    }

    public String toString() {
        StringBuilder str = new StringBuilder();
        for (int i = Constants.LONGEST_WALL - 1; i >= 0; i--) {
            if (i >= attackerCards.size()) {
                str.append(Constants.CARD_SPACE).append(" ");
            } else {
                str.append(attackerCards.get(i).toString()).append(" ");
            }
        }
        str.append(status.leftSymbol).append(wallNum).append(status.rightSymbol).append(" ");
        str.append("  ".repeat(Constants.LONGEST_WALL - length));
        for (int i = 0; i < length; i++) {
            str.append("[").append(pattern.getSymbol()).append("] ");
        }
        str.append("  ".repeat(Constants.LONGEST_WALL - length));
        str.append(status.leftSymbol).append(wallNum).append(status.rightSymbol);

        for (Card card : defenderCards) {
            str.append(" ").append(card.toString());
        }

        return str.toString();
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
        Set<Color> colors = new TreeSet<>();
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

    public void reset() {
        attackerCards.clear();
        defenderCards.clear();
        status = Status.INTACT;
        length = intactLength;
        pattern = intactPattern;
    }
}
