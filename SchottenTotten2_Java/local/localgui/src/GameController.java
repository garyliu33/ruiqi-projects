import java.util.List;

public class GameController {
    private final Game game;
    private final GameView gameView;

    private Phase currentPhase;

    private enum Phase {
        ATTACKER_TURN,
        DEFENDER_TURN,
        GAME_OVER
    }

    public GameController(Game game) {
        this.game = game;
        this.gameView = new GameView(game, this::onWallClicked);
    }

    public void runGame() {
        game.setup();
        gameView.update();
        start();
    }

    private void start() {
        currentPhase = Phase.ATTACKER_TURN;
    }

    private Card getSelectedCard() {
        if (currentPhase == Phase.ATTACKER_TURN) {
            return gameView.getSelectedAttackerCard();
        } else {
            return gameView.getSelectedDefenderCard();
        }
    }

    public void onWallClicked(Wall wall) {
        Card card = getSelectedCard();
        if (card != null) {
            PlayResult result = wall.playCard(card, currentPhase == Phase.ATTACKER_TURN);
            if (result.getResultType() == PlayResult.Type.SUCCESS) {
                game.getDiscard().addAll(result.getToDiscard());
                if (currentPhase == Phase.ATTACKER_TURN) {
                    game.getAttacker().getHand().remove(card);
                    gameView.unselectAttackerCard();
                    game.getAttacker().draw(game.getDeck());
                    game.declareControl();
                    gameView.update();
                    Winner winner = game.getWinner(true);
                    if (winner != Winner.NONE) {
                        currentPhase = Phase.GAME_OVER;
                        gameView.displayWinner(winner);
                        return;
                    }
                    currentPhase = Phase.DEFENDER_TURN;
                } else if (currentPhase == Phase.DEFENDER_TURN) {
                    game.getDefender().getHand().remove(card);
                    gameView.unselectDefenderCard();
                    game.getDefender().draw(game.getDeck());
                    game.declareControl();
                    game.getDefender().setUseCauldron(false);
                    gameView.update();
                    Winner winner = game.getWinner(false);
                    if (winner != Winner.NONE) {
                        currentPhase = Phase.GAME_OVER;
                        gameView.displayWinner(winner);
                        return;
                    }
                    currentPhase = Phase.ATTACKER_TURN;
                }
            } else if (result.getResultType() == PlayResult.Type.ACTION) {
                List<Card> toDiscard = result.getToDiscard();
                if (!toDiscard.isEmpty()) {
                    game.getDiscard().addAll(toDiscard);
                    if (currentPhase == Phase.DEFENDER_TURN) {
                        game.getDefender().setUseCauldron(true);
                    }
                    gameView.update();
                }
            }
        }
    }

    public GameView getGameView() {
        return gameView;
    }
}
