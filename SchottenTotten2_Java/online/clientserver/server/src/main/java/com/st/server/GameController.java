package com.st.server;

import com.st.common.*;

import java.util.ArrayList;
import java.util.List;
import java.util.Set;

/**
 * Manages the game logic on the server side. This is a simplified version
 * of the hostgui's GameController, without any Swing dependencies.
 */
public class GameController {
    private GameState fullGameState;
    private Role currentTurn;

    public void startGame() {
        this.fullGameState = new GameState();
        Deck deck = fullGameState.getDeck();

        Player attacker = new Player();
        Player defender = new Player();
        for (int i = 0; i < Constants.HAND_SIZE; i++) {
            attacker.draw(deck);
            defender.draw(deck);
        }
        fullGameState.getAttackerHand().addAll(attacker.getHand().getCards());
        fullGameState.getDefenderHand().addAll(defender.getHand().getCards());
        currentTurn = Role.ATTACKER; // Attacker always starts
    }

    public void processMove(ClientMove move, Role playerRole) {
        if (playerRole != currentTurn) {
            System.out.println("Not player " + playerRole + "'s turn.");
            return;
        }

        Card card = move.card();
        int wallIndex = move.wallIndex();
        boolean isAttacker = playerRole == Role.ATTACKER;

        Wall[] walls = fullGameState.getWalls();
        PlayResult result = walls[wallIndex].playCard(card, isAttacker);

        if (result.getResultType() == PlayResult.Type.SUCCESS) {
            Set<Card> hand = isAttacker ? fullGameState.getAttackerHand() : fullGameState.getDefenderHand();
            hand.remove(card);
            Card drawnCard = fullGameState.getDeck().pop();
            if (drawnCard != null) {
                hand.add(drawnCard);
            }

            fullGameState.getDiscard().values().stream().flatMap(List::stream).close();
            result.getToDiscard().forEach(c -> fullGameState.getDiscard().get(c.getColor()).add(c));

            declareControl();

            // Switch turns
            currentTurn = (currentTurn == Role.ATTACKER) ? Role.DEFENDER : Role.ATTACKER;

            fullGameState = new GameState(
                    fullGameState.getAttackerHand(), fullGameState.getDefenderHand(), fullGameState.getWalls(),
                    fullGameState.getDeck(), fullGameState.getDiscard(), false,
                    fullGameState.getCauldronCount(), false, // Reset usedCauldron
                    false, getWinner(), card
            );

        } else if (result.getResultType() == PlayResult.Type.ACTION) {
            result.getToDiscard().forEach(c -> fullGameState.getDiscard().get(c.getColor()).add(c));
            boolean usedCauldron = !isAttacker;
            int cauldronCount = fullGameState.getCauldronCount() - (usedCauldron ? 1 : 0);

            fullGameState = new GameState(
                    fullGameState.getAttackerHand(), fullGameState.getDefenderHand(), fullGameState.getWalls(),
                    fullGameState.getDeck(), fullGameState.getDiscard(), false,
                    cauldronCount, usedCauldron, false, getWinner(), card
            );
        }
    }

    private void declareControl() {
        List<Card> remainingCards = new ArrayList<>();
        Set<Card> discardedOrOnBoard = fullGameState.getDiscard().values().stream().flatMap(List::stream).collect(java.util.stream.Collectors.toSet());
        for (Wall wall : fullGameState.getWalls()) {
            for (Card card : wall.getAttackerCards()) discardedOrOnBoard.add(card);
            for (Card card : wall.getDefenderCards()) discardedOrOnBoard.add(card);
        }

        for (var card : Constants.ALL_CARDS) {
            if (!discardedOrOnBoard.contains(card)) {
                remainingCards.add(card);
            }
        }
        for (Wall wall : fullGameState.getWalls()) {
            Set<Card> toDiscard = wall.declareControl(remainingCards);
            toDiscard.forEach(c -> fullGameState.getDiscard().get(c.getColor()).add(c));
        }
    }

    public GameState createGameStateForPlayer(Role playerRole) {
        boolean isAttacker = playerRole == Role.ATTACKER;
        Set<Card> hostHand = isAttacker ? fullGameState.getAttackerHand() : fullGameState.getDefenderHand();
        Set<Card> clientHand = isAttacker ? fullGameState.getAttackerHand() : fullGameState.getDefenderHand();

        return new GameState(
                hostHand, clientHand, fullGameState.getWalls(),
                fullGameState.getDeck(), fullGameState.getDiscard(),
                currentTurn != playerRole, fullGameState.getCauldronCount(),
                fullGameState.hasUsedCauldron(), !isAttacker,
                fullGameState.getWinner(), fullGameState.getLastPlayedCard());
    }

    public GameState getFullGameState() {
        return fullGameState;
    }

    private Winner getWinner() {
        int numDamaged = 0;
        for (Wall wall : fullGameState.getWalls()) {
            if (wall.getStatus() == Wall.Status.BROKEN) return Winner.ATTACKER;
            if (wall.getStatus() == Wall.Status.DAMAGED) numDamaged++;
        }
        return numDamaged >= 4 ? Winner.ATTACKER : Winner.NONE;
    }
}
