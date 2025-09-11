package com.st.client;

import java.awt.Color;
import java.awt.Component;
import java.awt.Dimension;
import java.awt.FlowLayout;
import java.awt.Graphics;
import java.awt.Graphics2D;
import java.awt.Rectangle;
import java.awt.RenderingHints;
import java.util.Set;

import javax.swing.JPanel;

import com.st.common.Card;
import com.st.common.Constants;

public class HandView extends JPanel {
    private CardContainer selectedCard = null;
    private final boolean glowing;

    public HandView(Set<Card> cards, boolean isAttacker, int cauldronCount, boolean hasUsedCauldron, boolean isOpponent, boolean isTurn) {
        this.glowing = isTurn;
        setLayout(new FlowLayout(FlowLayout.CENTER, 0, 0));
        setPreferredSize(new Dimension(Constants.WINDOW_WIDTH, Constants.CARD_HEIGHT + Constants.POP_OFFSET));
        setMaximumSize(new Dimension(Constants.WINDOW_WIDTH, Constants.CARD_HEIGHT + Constants.POP_OFFSET));
        if (isOpponent) {
            for (int i = 0; i < cards.size(); i++) {
                add(new CardBackView());
            }
        } else {
            for (Card card : cards) {
                add(new CardContainer(card, this));
            }
        }

        if (isAttacker) {
            if (!isOpponent) {
                add(new CardContainer(Card.RETREAT, this));
            }
        } else if (!hasUsedCauldron) {
            for (int i = 0; i < cauldronCount; i++) {
                add(isOpponent ? new CardView(Card.CAULDRON, false) : new CardContainer(Card.CAULDRON, this));
            }
        }
    }

    protected void paintComponent(Graphics g) {
        super.paintComponent(g);

        if (glowing && getComponentCount() > 0) {
            Rectangle glowRect = null;
            for (Component card : getComponents()) {
                Rectangle bounds = card.getBounds();
                if (glowRect == null) {
                    glowRect = new Rectangle(bounds);
                } else {
                    glowRect = glowRect.union(bounds);
                }
            }

            Graphics2D g2 = (Graphics2D) g.create();
            g2.setRenderingHint(RenderingHints.KEY_ANTIALIASING, RenderingHints.VALUE_ANTIALIAS_ON);

            glowRect.grow(Constants.POP_OFFSET, Constants.POP_OFFSET);
            g2.setColor(new Color(255, 215, 0, 64));
            g2.fillRoundRect(glowRect.x, glowRect.y, glowRect.width, glowRect.height, 20, 20);

            g2.dispose();
        }
    }

    public void notifyCardClicked(CardContainer clickedCard) {
        if (selectedCard != null && selectedCard != clickedCard) {
            selectedCard.unPop();
        } else if (selectedCard == clickedCard) {
            unselectCard();
            return;
        }
        selectedCard = clickedCard;
    }

    public Card getSelectedCard() {
        return selectedCard == null ? null : selectedCard.getCard();
    }

    public void unselectCard() {
        selectedCard = null;
    }
}
