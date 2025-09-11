package com.st.client;

import java.awt.BasicStroke;
import java.awt.Color;
import java.awt.Dimension;
import java.awt.Font;
import java.awt.FontMetrics;
import java.awt.Graphics;
import java.awt.Graphics2D;
import java.awt.GraphicsEnvironment;
import java.awt.Stroke;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;
import java.awt.image.BufferedImage;
import java.io.IOException;
import java.util.Map;
import java.util.Objects;

import javax.imageio.ImageIO;
import javax.swing.BorderFactory;
import javax.swing.JPanel;

import com.st.common.CardColor;
import com.st.common.Constants;
import com.st.common.Card;

public class CardView extends JPanel {
    private final Card card;
    private final boolean isLastPlayed;

    private static BufferedImage retreatCard;
    private static BufferedImage cauldronCard;

    private static Map<CardColor, String> cardIconMap = Map.of(
            CardColor.RED, "♥",
            CardColor.BLUE, "♠",
            CardColor.YELLOW, "★",
            CardColor.GREEN, "♦",
            CardColor.GRAY, "♣");

    static {
        try {
            retreatCard = ImageIO.read(Objects.requireNonNull(CardView.class.getResource("/retreat.png")));
        } catch (IOException | IllegalArgumentException | NullPointerException e) {
            e.printStackTrace();
            retreatCard = null;
        }

        try {
            cauldronCard = ImageIO.read(Objects.requireNonNull(CardView.class.getResource("/cauldron.png")));
        } catch (IOException | IllegalArgumentException | NullPointerException e) {
            e.printStackTrace();
            cauldronCard = null;
        }
    }

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
            g2.setFont(getFont(Constants.CARD_FONT_SIZE));
            FontMetrics fm = g2.getFontMetrics();
            int padding = 5;
            if (card.equals(Card.RETREAT)) {
                if (retreatCard != null) {
                    g2.drawImage(retreatCard, 0, 0, Constants.CARD_WIDTH, Constants.CARD_HEIGHT, this);
                } else {
                    g2.drawString("RETR", padding, fm.getAscent() + padding);
                    g2.drawString("EAT", padding, fm.getAscent() + padding + fm.getHeight());
                }
            } else if (card.equals(Card.CAULDRON)) {
                if (cauldronCard != null) {
                    g2.drawImage(cauldronCard, 0, 0, Constants.CARD_WIDTH, Constants.CARD_HEIGHT, this);
                } else {
                    g2.drawString("CAUL", padding, fm.getAscent() + padding);
                    g2.drawString("DRON", padding, fm.getAscent() + padding + fm.getHeight());
                }
            } else {
                String icon = cardIconMap.get(card.getColor());

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
                g2.drawString(icon, padding * 2 + fm.stringWidth(text), y);

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
                g2.drawString(icon, padding * 2 + fm.stringWidth(text), y);

                g2.dispose();
            }

            if (isLastPlayed) {
                setBorder(BorderFactory.createLineBorder(Color.ORANGE, 3));
            }
        }
    }

    public Card getCard() {
        return card;
    }

    public Font getFont(int fontSize) {
        GraphicsEnvironment ge = GraphicsEnvironment.getLocalGraphicsEnvironment();
        String[] fontNames = ge.getAvailableFontFamilyNames();

        // Check for a high-quality physical font
        for (String name : fontNames) {
            if ("Segoe UI Symbol".equals(name) || "DejaVu Sans".equals(name) || "Noto Sans".equals(name)
                    || "Apple Symbols".equals(name)) {
                return new Font(name, Font.BOLD, fontSize);
            }
        }

        // Fall back to the logical font
        return new Font("Arial", Font.BOLD, fontSize);
    }
}