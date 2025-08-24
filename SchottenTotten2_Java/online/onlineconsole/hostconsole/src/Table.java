import java.util.*;

public class Table {
    private static Table instance;
    private final Wall[] board;
    private final Deck deck;
    private final Discard discard;
    private int cauldronCount;

    private Table(Deck deck, Discard discard) { // pass in deck/discard instance
        board = new Wall[Constants.NUM_WALLS];
        for (int i = 0; i < Constants.NUM_WALLS; i++) {
            board[i] = new Wall(Constants.WALL_LENGTHS[i], Constants.DAMAGED_WALL_LENGTHS[i], Constants.WALL_PATTERNS[i], Constants.DAMAGED_WALL_PATTERNS[i], i + 1);
        }
        this.deck = deck;
        this.discard = discard;
        cauldronCount = Constants.NUM_CAULDRONS;
    }

    public static synchronized Table getInstance() {
        if (instance == null) {
            instance = new Table(Deck.getInstance(), Discard.getInstance());
        }
        return instance;
    }

    public String toString() {
        StringBuilder str = new StringBuilder();
        str.append("\n");
        str.append((Constants.CARD_SPACE + " ").repeat(Constants.LONGEST_WALL).substring(8));
        str.append("ATTACKER").append(" ".repeat(board[0].getLeftSymbolLength()));
        str.append(" ".repeat(Constants.LONGEST_WALL * 2)).append("DECK:");
        if (deck.size() < 10) {
            str.append("0");
        }
        str.append(deck.size()).append(" ".repeat(Constants.LONGEST_WALL * 2));
        str.append(" ".repeat(board[0].getRightSymbolLength())).append("DEFENDER ");
        str.append(Constants.CAULDRON.repeat(cauldronCount));
        str.append("\n");

        for (Wall wall : board) {
            str.append(wall).append("\n");
        }

        str.append("-".repeat((Constants.CARD_SPACE.length() + 1) * Constants.LONGEST_WALL + board[0].getLeftSymbolLength() + Constants.LONGEST_WALL * 2));
        str.append("DISCARD");
        str.append("-".repeat((Constants.CARD_SPACE.length() + 1) * Constants.LONGEST_WALL + board[0].getRightSymbolLength() + Constants.LONGEST_WALL * 2));
        str.append("\n");
        if (discard.isEmpty()) {
            str.append("\n");
        } else {
            str.append(discard);
        }
        str.append("-".repeat(2 * (Constants.CARD_SPACE.length() + 1) * Constants.LONGEST_WALL));
        str.append("-".repeat(board[0].getLeftSymbolLength() + board[0].getRightSymbolLength() + Constants.LONGEST_WALL * 4));
        str.append("-------");
        return str.toString();
    }

    public void setup(Player attacker, Player defender) {
        clear();
        cauldronCount = Constants.NUM_CAULDRONS;
        deck.reset();
        discard.clear();
        for (int i = 0; i < Constants.HAND_SIZE; i++) {
            attacker.draw();
            defender.draw();
        }
    }

    private void clear() {
        for (Wall wall : board) {
            wall.reset();
        }
    }

    public Played playCard(Card card, int wall, boolean isAttacker) {
        return board[wall - 1].playCard(card, isAttacker);
    }

    public boolean retreat(int wall) {
        List<Card> cards = board[wall - 1].retreat();
        if (!cards.isEmpty()) {
            discard.addAll(cards);
            Display.toBothln(toString());
            Display.toBothln("Attacker retreated from wall " + wall + ".");
            return true;
        }
        return false;
    }

    public boolean cauldron(int wall) {
        Card card = board[wall - 1].cauldron();
        if (card != null) {
            discard.add(card);
            cauldronCount--;
            Display.toBothln(toString());
            String str = "Defender used cauldron on wall " + wall + ".\n" + cauldronCount + " cauldron";
            if (cauldronCount != 1) {
                str += "s";
            }
            str += " remaining.";
            Display.toBothln(str);
            return true;
        }
        return false;
    }

    public void declareControl() {
        List<Card> remainingCards = new ArrayList<>();
        for (Card card : Constants.ALL_CARDS) {
            if (!discard.contains(card) && !onBoard(card)) {
                remainingCards.add(card);
            }
        }

        for (Wall wall : board) {
            if (wall.declareControl(remainingCards)) {
                discard.addAll(wall.damage());
            }
        }
    }

    public boolean onBoard(Card card) {
        for (Wall wall : board) {
            if (wall.contains(card)) {
                return true;
            }
        }
        return false;
    }

    public Winner won(boolean checkDeck) {
        int numDamaged = 0;
        for (Wall wall : board) {
            if (wall.isBroken()) {
                return Winner.ATTACKER;
            } else if (wall.isDamaged()) {
                numDamaged++;
            }
        }
        if (numDamaged >= 4) {
            return Winner.ATTACKER;
        }
        if (checkDeck && (deck.isEmpty() || defenderSideFull())) {
            return Winner.DEFENDER;
        }

        return Winner.NONE;
    }

    private boolean defenderSideFull() {
        for (Wall wall : board) {
            if (wall.defenderHasSpace()) {
                return false;
            }
        }
        return true;
    }

    public int getCauldronCount() {
        return cauldronCount;
    }
}