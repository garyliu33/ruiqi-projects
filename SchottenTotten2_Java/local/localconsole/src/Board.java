import java.util.*;

public class Board {
    private static Board instance;
    private final Wall[] board;
    private Deck deck;
    private Discard discard;
    private int cauldronCount;

    private Board() {
        board = new Wall[Constants.numWalls];
        for (int i = 0; i < Constants.numWalls; i++) {
            board[i] = new Wall(Constants.wallLengths[i], Constants.damagedWallLengths[i], Constants.wallPatterns[i], Constants.damagedWallPatterns[i], i + 1);
        }
        deck = Deck.getInstance();
        discard = Discard.getInstance();
        cauldronCount = Constants.numCauldrons;
    }

    public static Board getInstance() {
        if (instance == null) {
            instance = new Board();
        }
        return instance;
    }

    public void display() {
        System.out.print((Constants.cardSpace() + " ").repeat(Constants.longestWall()).substring(8));
        System.out.print("ATTACKER" + " ".repeat(Constants.leftWalls[0].length()));
        System.out.print(" ".repeat(Constants.longestWall() * 2) + "DECK:");
        if (deck.size() < 10) {
            System.out.print("0");
        }
        System.out.print(deck.size() + " ".repeat(Constants.longestWall() * 2));
        System.out.print(" ".repeat(Constants.rightWalls[0].length()) + "DEFENDER ");
        for (int i = 0; i < cauldronCount; i++) {
            System.out.print(Constants.CAULDRON);
        }
        System.out.println();
        for (Wall wall : board) {
            wall.display();
        }

        System.out.print("-".repeat((Constants.cardSpace().length() + 1) * Constants.longestWall()));
        System.out.print("-".repeat(Constants.leftWalls[0].length() + Constants.longestWall() * 2));
        System.out.print("DISCARD");
        System.out.print("-".repeat((Constants.cardSpace().length() + 1) * Constants.longestWall()));
        System.out.print("-".repeat(Constants.rightWalls[0].length() + Constants.longestWall() * 2));
        discard.display();
        System.out.println();
        System.out.print("-".repeat((Constants.cardSpace().length() + 1) * Constants.longestWall()));
        System.out.print("-".repeat(Constants.leftWalls[0].length() + Constants.longestWall() * 2));
        System.out.print("-------");
        System.out.print("-".repeat((Constants.cardSpace().length() + 1) * Constants.longestWall()));
        System.out.println("-".repeat(Constants.rightWalls[0].length() + Constants.longestWall() * 2));
    }

    public void setup(Player attacker, Player defender) {
        deck.shuffle();
        for (int i = 0; i < Constants.handSize; i++) {
            attacker.draw();
            defender.draw();
        }
    }

    public boolean playCard(Card card, int wall, boolean attacker) {
        int i = board[wall - 1].playCard(card, attacker);
        if (i == -1) {
            return false;
        } else if (i != 0) {
            discard.add(new Card(Constants.colors.get(i - 1), 0));
            discard.add(new Card(Constants.colors.get(i - 1), 11));
        }
        return true;
    }

    public void retreat(int wall) {
        discard.addAll(board[wall - 1].retreat());
        display();
    }

    public boolean cauldron(int wall) {
        Card card = board[wall - 1].cauldron();
        if (card != null) {
            discard.add(card);
            cauldronCount--;
            display();
            System.out.print(cauldronCount + " cauldron");
            if (cauldronCount != 1) {
                System.out.print("s");
            }
            System.out.println(" remaining");
            return true;
        }
        return false;
    }

    public void declareControl() {
        List<Card> remainingCards = new ArrayList<>();
        for (Card card : Constants.allCards()) {
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

    public int won() {
        int numDamaged = 0;
        for (Wall wall : board) {
            if (wall.isBroken()) {
                return Constants.attackerWins;
            } else if (wall.isDamaged()) {
                numDamaged++;
            }
        }
        if (numDamaged >= 4) {
            return Constants.attackerWins;
        }
        if (deck.isEmpty()) {
            return Constants.defenderWins;
        }
        if (defenderSideFull()) {
            return Constants.defenderWins;
        }
        return Constants.noWinner;
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
