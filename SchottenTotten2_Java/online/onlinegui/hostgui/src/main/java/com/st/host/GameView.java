package com.st.host;

import java.util.function.Consumer;

import javax.swing.Box;
import javax.swing.BoxLayout;
import javax.swing.JPanel;

import com.st.common.Card;
import com.st.common.GameState;
import com.st.common.Wall;

public class GameView extends JPanel {
    private HandView hostHandView;
    private final GameState gameState;

    public GameView(GameState gameState, Consumer<Wall> onWallClicked) {
        setLayout(new BoxLayout(this, BoxLayout.Y_AXIS));
        this.gameState = gameState;
        updateLayout(onWallClicked);
    }

    public Card getSelectedCard() {
        return hostHandView.getSelectedCard();
    }

    public void unselectCard() {
        hostHandView.unselectCard();
    }

    public void updateLayout(Consumer<Wall> onWallClicked) {
        removeAll();
        hostHandView = new HandView(gameState.getHostHand(), !gameState.isClientAttacker(), gameState.getCauldronCount(), gameState.hasUsedCauldron(), false, !gameState.isClientTurn());
        HandView clientHandView = new HandView(gameState.getClientHand(), gameState.isClientAttacker(), gameState.getCauldronCount(), gameState.hasUsedCauldron(), true, gameState.isClientTurn());
        TableView tableView = new TableView(gameState.getWalls(), gameState.getDeckSize(), gameState.getDiscard(), onWallClicked, !gameState.isClientAttacker(), gameState.getLastPlayedCard());

        add(clientHandView);
        add(Box.createVerticalGlue());
        add(tableView);
        add(Box.createVerticalGlue());
        add(hostHandView);

        revalidate();
        repaint();
    }
}
