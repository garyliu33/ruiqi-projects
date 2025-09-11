package com.st.client;

import javax.swing.*;

import java.util.function.Consumer;

import com.st.common.Card;
import com.st.common.GameState;
import com.st.common.Wall;

public class GameView extends JPanel {
    private HandView clientHandView;
    private final GameState gameState;

    public GameView(GameState gameState, Consumer<Wall> onWallClicked) {
        setLayout(new BoxLayout(this, BoxLayout.Y_AXIS));
        this.gameState = gameState;
        updateLayout(onWallClicked);

    }

    public Card getSelectedCard() {
        return clientHandView.getSelectedCard();
    }

    public void unselectCard() {
        clientHandView.unselectCard();
    }

    public void updateLayout(Consumer<Wall> onWallClicked) {
        removeAll();
        HandView hostHandView = new HandView(gameState.isClientAttacker() ? gameState.getDefenderHand() : gameState.getAttackerHand(),
                !gameState.isClientAttacker(),
                gameState.getCauldronCount(), gameState.hasUsedCauldron(), true,
                !gameState.isClientTurn());
        clientHandView = new HandView(gameState.isClientAttacker() ? gameState.getAttackerHand() : gameState.getDefenderHand(),
                gameState.isClientAttacker(),
                gameState.getCauldronCount(), gameState.hasUsedCauldron(), false,
                gameState.isClientTurn());
        TableView tableView = new TableView(gameState.getWalls(), gameState.getDeck().size(),
                gameState.getDiscard(), onWallClicked, !gameState.isClientAttacker(),
                gameState.getLastPlayedCard());

        add(hostHandView);
        add(Box.createVerticalGlue());
        add(tableView);
        add(Box.createVerticalGlue());
        add(clientHandView);

        revalidate();
        repaint();
    }
}
