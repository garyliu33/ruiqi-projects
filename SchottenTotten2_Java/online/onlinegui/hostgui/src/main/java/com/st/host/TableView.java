package com.st.host;

import java.util.List;
import java.util.Map;
import java.util.function.Consumer;

import javax.swing.Box;
import javax.swing.BoxLayout;
import javax.swing.JPanel;

import com.st.common.Card;
import com.st.common.CardColor;
import com.st.common.Wall;

public class TableView extends JPanel {

    public TableView(Wall[] walls, int deckSize, Map<CardColor, List<Card>> discard, Consumer<Wall> onWallClicked, boolean hostIsAttacker, Card lastPlayedCard) {
        setLayout(new BoxLayout(this, BoxLayout.X_AXIS));

        BoardView boardView = new BoardView(walls, onWallClicked, hostIsAttacker, lastPlayedCard);
        DeckView deckView = new DeckView(deckSize);
        DiscardView discardView = new DiscardView(discard, lastPlayedCard);

        add(Box.createHorizontalGlue());
        add(deckView);
        add(Box.createHorizontalGlue());
        add(boardView);
        add(Box.createHorizontalGlue());
        add(discardView);
        add(Box.createHorizontalStrut(20));
    }
}
