package com.st.host;

import java.io.*;
import java.util.List;

public class GameController {
    private final Game game;
    private GameView gameView;
    private final Role hostRole;
    private boolean playAgain;
    private Card lastPlayedCard;
    private Phase currentPhase;
    private final Network network;

    private enum Phase {
        HOST_TURN,
        CLIENT_TURN,
        GAME_OVER
    }

    public GameController(Game game, Role hostRole, Network network) {
        this.game = game;
        this.gameView = new GameView(createGameState(false), this::onWallClicked);
        this.hostRole = hostRole;
        this.network = network;
        this.network.setMoveHandler(move -> {
            if (currentPhase == Phase.CLIENT_TURN) {
                try {
                    processMove(move);
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
        });
        playAgain = false;
    }

    public void startGame() {
        game.setup();
        currentPhase = hostRole == Role.ATTACKER ? Phase.HOST_TURN : Phase.CLIENT_TURN;
        displayGameState(false);
    }

    private void processMove(ClientMove move) throws IOException {
        Card card = move.card();
        int wallIndex = move.wallIndex();

        if (currentPhase == Phase.CLIENT_TURN) {
            PlayResult result = game.board().playCard(wallIndex, card, hostRole == Role.DEFENDER);
            if (result.getResultType() == PlayResult.Type.SUCCESS) {
                lastPlayedCard = card;
                game.discard().addAll(result.getToDiscard());
                getClient().getHand().remove(card);
                getClient().draw(game.deck());
                game.declareControl();
                getClient().setUseCauldron(false);
                currentPhase = Phase.HOST_TURN;
                displayGameState(hostRole == Role.DEFENDER);
            } else if (result.getResultType() == PlayResult.Type.ACTION) {
                List<Card> toDiscard = result.getToDiscard();
                if (!toDiscard.isEmpty()) {
                    game.discard().addAll(toDiscard);
                    if (hostRole == Role.ATTACKER) {
                        game.defender().setUseCauldron(true);
                    }
                    displayGameState(false);
                }
            }
        }
    }

    public void onWallClicked(Wall wall) {
        if (currentPhase == Phase.HOST_TURN) {
            Card card = getSelectedCard();
            if (card != null) {
                PlayResult result = wall.playCard(card, hostRole == Role.ATTACKER);
                if (result.getResultType() == PlayResult.Type.SUCCESS) {
                    lastPlayedCard = card;
                    game.discard().addAll(result.getToDiscard());
                    getHost().getHand().remove(card);
                    gameView.unselectCard();
                    getHost().draw(game.deck());
                    game.declareControl();
                    getHost().setUseCauldron(false);
                    currentPhase = Phase.CLIENT_TURN;
                    displayGameState(hostRole == Role.ATTACKER);
                } else if (result.getResultType() == PlayResult.Type.ACTION) {
                    List<Card> toDiscard = result.getToDiscard();
                    if (!toDiscard.isEmpty()) {
                        game.discard().addAll(toDiscard);
                        if (hostRole == Role.DEFENDER) {
                            game.defender().setUseCauldron(true);
                        }
                        displayGameState(false);
                    }
                }
            }
        }
    }

    private Card getSelectedCard() {
        if (currentPhase == Phase.HOST_TURN) {
            return gameView.getSelectedCard();
        }
        return null;
    }

    private Player getClient() {
        return hostRole == Role.DEFENDER ? game.attacker() : game.defender();
    }

    private Player getHost() {
        return hostRole == Role.ATTACKER ? game.attacker() : game.defender();
    }

    private GameState createGameState(boolean checkDeck) {
        return new GameState(getHost().getHand().getCards(),
                getClient().getHand().getCards(),
                game.board().getWalls(),
                game.deck().size(),
                game.discard().getCardsByColor(),
                currentPhase == Phase.CLIENT_TURN,
                game.defender().getCauldronCount(),
                game.defender().hasUsedCauldron(),
                hostRole == Role.DEFENDER,
                game.getWinner(checkDeck),
                lastPlayedCard);
    }

    public void displayGameState(boolean checkDeck) {
        GameState state = createGameState(checkDeck);
        network.sendGameState(state);
        gameView = new GameView(state, this::onWallClicked);
        HostGUI.displayGameState();

        if (state.getWinner() != Winner.NONE) {
            playAgain = HostGUI.showRematchDialog(state.getWinner());
            currentPhase = Phase.GAME_OVER;
        }
    }

    public void updateGameView() {
        gameView.updateLayout(this::onWallClicked);
    }

    public boolean playAgain() {
        return playAgain;
    }

    public boolean gameOver() {
        return currentPhase == Phase.GAME_OVER;
    }

    public GameView getGameView() {
        return gameView;
    }
}
