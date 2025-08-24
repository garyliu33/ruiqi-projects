import java.util.*;

public class Game {
    private final Player attacker;
    private final Player defender;
    private final Board board;
    private final Deck deck;
    private final Discard discard;

    public Game(Player attacker, Player defender, Board board, Deck deck, Discard discard) {
        this.attacker = attacker;
        this.defender = defender;
        this.board = board;
        this.deck = deck;
        this.discard = discard;
    }

    public void setup() {
        deck.reset();
        discard.clear();
        board.clear();
        for (int i = 0; i < Constants.HAND_SIZE; i++) {
            attacker.draw(deck);
            defender.draw(deck);
        }
    }

    public void declareControl() {
        List<Card> remainingCards = new ArrayList<>();
        for (Card card : Constants.ALL_CARDS) {
            if (!discard.contains(card) && !board.contains(card)) {
                remainingCards.add(card);
            }
        }
        discard.addAll(board.declareControl(remainingCards));
    }

    public Winner getWinner(boolean checkDeck) {
        int numDamaged = 0;
        Wall[] walls = board.getWalls();
        for (Wall wall : walls) {
            switch (wall.getStatus()) {
                case BROKEN -> {
                    return Winner.ATTACKER;
                }
                case DAMAGED -> numDamaged++;
            }
        }

        if (numDamaged >= 4) {
            return Winner.ATTACKER;
        }

        if (checkDeck && (deck.isEmpty() || board.defenderSideFull())) {
            return Winner.DEFENDER;
        }

        return Winner.NONE;
    }

    public Player getAttacker() {
        return attacker;
    }

    public Player getDefender() {
        return defender;
    }

    public Board getBoard() {
        return board;
    }

    public Deck getDeck() {
        return deck;
    }

    public Discard getDiscard() {
        return discard;
    }
}
