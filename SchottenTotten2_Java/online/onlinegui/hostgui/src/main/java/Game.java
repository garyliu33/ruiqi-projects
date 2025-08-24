import java.util.*;

public record Game(Player attacker, Player defender, Board board, Deck deck, Discard discard) {

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
}
