package com.st.client;

import java.awt.Color;
import java.awt.Dimension;
import java.awt.Font;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;
import java.util.List;
import java.util.function.Consumer;

import javax.swing.BorderFactory;
import javax.swing.JLabel;
import javax.swing.JPanel;
import javax.swing.SwingConstants;

import com.st.common.Card;
import com.st.common.CardView;
import com.st.common.Constants;
import com.st.common.Wall;

public class WallView extends JPanel {
    public WallView(Wall wall, Consumer<Wall> onWallClicked, boolean hostIsAttacker, Card lastPlayedCard) {
        setLayout(null);
        setPreferredSize(new Dimension(Constants.CARD_WIDTH, Constants.WALL_OVERALL_HEIGHT));
        setOpaque(true);

        List<Card> topCards = hostIsAttacker ? wall.getAttackerCards() : wall.getDefenderCards();
        for (int i = topCards.size() - 1; i >= 0; i--) {
            Card card = topCards.get(i);
            CardView cardView;
            if (card.equals(lastPlayedCard)) {
                cardView = new CardView(card, true);
            } else {
                cardView = new CardView(card, false);
            }
            cardView.setBounds(0, (Constants.WALL_OVERALL_HEIGHT - Constants.WALL_LABEL_HEIGHT) / 2 - i * Constants.OVERLAP - Constants.OVERLAP / 2 - Constants.CARD_HEIGHT, Constants.CARD_WIDTH, Constants.CARD_HEIGHT);
            add(cardView);
        }

        if (wall.getStatus() != Wall.Status.BROKEN) {
            JLabel label = new JLabel(("[" + wall.getPattern().getSymbol() + "]").repeat(wall.getLength()), SwingConstants.CENTER);
            label.setFont(new Font("Arial", Font.PLAIN, 15));
            label.setOpaque(true);
            if (wall.getStatus() == Wall.Status.DAMAGED) {
                label.setBorder(BorderFactory.createDashedBorder(Color.BLACK, 2, 2, 4, false));
            } else {
                label.setBorder(BorderFactory.createLineBorder(Color.BLACK, 2));
            }
            label.setBounds(0, (Constants.WALL_OVERALL_HEIGHT - Constants.WALL_LABEL_HEIGHT) / 2, Constants.WALL_WIDTH, Constants.WALL_LABEL_HEIGHT);
            add(label);
        }

        List<Card> bottomCards = hostIsAttacker ? wall.getDefenderCards() : wall.getAttackerCards();
        for (int i = bottomCards.size() - 1; i >= 0; i--) {
            Card card = bottomCards.get(i);
            CardView cardView;
            if (card.equals(lastPlayedCard)) {
                cardView = new CardView(card, true);
            } else {
                cardView = new CardView(card, false);
            }
            cardView.setBounds(0, (Constants.WALL_OVERALL_HEIGHT + Constants.WALL_LABEL_HEIGHT) / 2 + i * Constants.OVERLAP + Constants.OVERLAP / 2, Constants.CARD_WIDTH, Constants.CARD_HEIGHT);
            add(cardView);
        }

        addMouseListener(new MouseAdapter() {
            @Override
            public void mouseClicked(MouseEvent e) {
                onWallClicked.accept(wall);
            }

            @Override
            public void mouseEntered(MouseEvent e) {
                setBorder(BorderFactory.createLineBorder(Color.GREEN, 3));
            }

            @Override
            public void mouseExited(MouseEvent e) {
                setBorder(BorderFactory.createEmptyBorder());
            }
        });
    }
}
