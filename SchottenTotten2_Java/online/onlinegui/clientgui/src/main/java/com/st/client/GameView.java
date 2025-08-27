package com.st.client;

import javax.swing.*;
import java.util.function.Consumer;

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
        HandView hostHandView = new HandView(gameState.getHostHand(), !gameState.isClientAttacker(), gameState.getCauldronCount(), gameState.getHasUsedCauldron(), true, !gameState.isClientTurn());
        clientHandView = new HandView(gameState.getClientHand(), gameState.isClientAttacker(), gameState.getCauldronCount(), gameState.getHasUsedCauldron(), false, gameState.isClientTurn());
        TableView tableView = new TableView(gameState.getWalls(), gameState.getDeckSize(), gameState.getDiscard(), onWallClicked, !gameState.isClientAttacker(), gameState.getLastPlayedCard());

        add(hostHandView);
        add(Box.createVerticalGlue());
        add(tableView);
        add(Box.createVerticalGlue());
        add(clientHandView);

        revalidate();
        repaint();
    }
}
