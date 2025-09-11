package com.st.client;

import com.st.common.Card;
import com.st.common.Constants;

import java.awt.Color;
import java.awt.Dimension;
import java.awt.Font;
import java.awt.Graphics;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;

import javax.swing.BorderFactory;
import javax.swing.JLabel;
import javax.swing.JPanel;
import javax.swing.SwingConstants;

public class CardView extends JPanel {
    private final Card card;

    public CardView(Card card, boolean highlighted) {
        this(card, highlighted, null);
    }

    public CardView(Card card, Runnable onCardClicked) {
        this(card, false, onCardClicked);
    }

    public CardView(Card card, boolean highlighted, Runnable onCardClicked) {
        this.card = card;
        setPreferredSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT));
        setMaximumSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT));
        setLayout(null);
        setOpaque(true);

        JLabel label = new JLabel(card.getValue() + "", SwingConstants.CENTER);
        label.setFont(new Font("Arial", Font.BOLD, Constants.CARD_FONT_SIZE));
        label.setBounds(0, 0, Constants.CARD_WIDTH, Constants.CARD_HEIGHT);
        add(label);

        if (highlighted) {
            setBorder(BorderFactory.createLineBorder(Color.YELLOW, 3));
        }

        if (onCardClicked != null) {
            addMouseListener(new MouseAdapter() {
                @Override
                public void mouseClicked(MouseEvent e) {
                    onCardClicked.run();
                }
            });
        }
    }

    public Card getCard() {
        return card;
    }

    protected void paintComponent(Graphics g) {
        super.paintComponent(g);
        g.setColor(card.getColor().getDisplayColor());
        g.fillRect(0, 0, Constants.CARD_WIDTH, Constants.CARD_HEIGHT);
        g.setColor(Color.BLACK);
        g.drawRect(0, 0, Constants.CARD_WIDTH - 1, Constants.CARD_HEIGHT - 1);
    }
}