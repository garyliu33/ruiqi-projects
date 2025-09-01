package com.st.client;

import java.awt.BasicStroke;
import java.awt.Color;
import java.awt.Dimension;
import java.awt.Font;
import java.awt.FontMetrics;
import java.awt.Graphics;
import java.awt.Graphics2D;
import java.awt.Stroke;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;

import javax.swing.BorderFactory;
import javax.swing.JPanel;

import com.st.common.Card;
import com.st.common.Constants;

public class CardView extends JPanel {
    private final Card card;
    private final boolean isLastPlayed;

    public CardView(Card card, boolean isLastPlayed) {
        this.card = card;
        this.isLastPlayed = isLastPlayed;
        setPreferredSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT));
        setBorder(BorderFactory.createLineBorder(Color.BLACK));
    }

    public CardView(Card card, Runnable onClick) {
        this.card = card;
        this.isLastPlayed = false;
        setPreferredSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT));
        setBorder(BorderFactory.createLineBorder(Color.BLACK));

        addMouseListener(new MouseAdapter() {
            @Override
            public void mouseClicked(MouseEvent e) {
                onClick.run();
            }

            @Override
            public void mouseEntered(MouseEvent e) {
                setBorder(BorderFactory.createLineBorder(Color.GREEN, 3));
            }

            @Override
            public void mouseExited(MouseEvent e) {
                setBorder(BorderFactory.createLineBorder(Color.BLACK));
            }
        });
    }

    protected void paintComponent(Graphics g) {
        super.paintComponent(g);
        Graphics2D g2 = (Graphics2D) g.create();

        if (card != null) {
            g2.setColor(Color.WHITE);
            g2.fillRect(0, 0, getWidth(), getHeight());
            g2.setColor(Color.BLACK);
            g2.drawRect(0, 0, getWidth() - 1, getHeight() - 1);

            g2.setColor(card.getColor().getDisplayColor());
            g2.setFont(new Font("Arial", Font.BOLD, Constants.CARD_FONT_SIZE));
            FontMetrics fm = g2.getFontMetrics();
            int padding = 5;
            if (card.equals(Card.RETREAT)) {
                g2.drawString("RETR", padding, fm.getAscent() + padding);
                g2.drawString("EAT", padding, fm.getAscent() + padding + fm.getHeight());
            } else if (card.equals(Card.CAULDRON)) {
                g2.drawString("CAUL", padding, fm.getAscent() + padding);
                g2.drawString("DRON", padding, fm.getAscent() + padding + fm.getHeight());
            } else {
                String text = card.getValue() + "";
                int y = fm.getAscent() + padding;

                g2.drawString(text, padding, y);
                if (text.equals("6") || text.equals("9")) {
                    int underlineY = y + 3;
                    Stroke oldStroke = g2.getStroke();
                    g2.setStroke(new BasicStroke(2));
                    g2.drawLine(padding + 2, underlineY, padding + fm.stringWidth(text) - 2, underlineY);
                    g2.setStroke(oldStroke);
                }

                g2.translate(getWidth(), getHeight());
                g2.rotate(Math.PI);
                g2.drawString(text, padding, y);

                if (text.equals("6") || text.equals("9")) {
                    int underlineY = y + 3;
                    Stroke oldStroke = g2.getStroke();
                    g2.setStroke(new BasicStroke(2));
                    g2.drawLine(padding + 2, underlineY, padding + fm.stringWidth(text) - 2, underlineY);
                    g2.setStroke(oldStroke);
                }

                g2.dispose();
            }
        }

        if (isLastPlayed) {
            setBorder(BorderFactory.createLineBorder(Color.ORANGE, 3));
        }
    }

    public Card getCard() {
        return card;
    }
}