package com.st.client;

import java.awt.Dimension;
import java.awt.FlowLayout;
import java.util.List;
import java.util.Map;

import javax.swing.JPanel;

import com.st.common.Card;
import com.st.common.CardColor;
import com.st.common.Constants;

public class DiscardView extends JPanel {
    public DiscardView(Map<CardColor, List<Card>> cardsByColor, Card lastPlayedCard) {
        setLayout(new FlowLayout(FlowLayout.CENTER, 10, 0));
        for (CardColor color : cardsByColor.keySet()) {
            List<Card> cards = cardsByColor.get(color);
            if (!cards.isEmpty()) {
                JPanel column = new JPanel();
                column.setLayout(null);
                column.setPreferredSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT + (Constants.VALUES.size() - 1) * Constants.OVERLAP));
                for (int i = cards.size() - 1; i >= 0; i--) {
                    Card card = cards.get(i);
                    var cardView = new CardView(card, card.equals(lastPlayedCard));
                    cardView.setBounds(0, card.getValue() * Constants.OVERLAP, Constants.CARD_WIDTH, Constants.CARD_HEIGHT);
                    column.add(cardView);
                }
                add(column);
            }
        }
    }
}
