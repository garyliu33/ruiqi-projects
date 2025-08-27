package com.st.client;

import javax.imageio.ImageIO;
import javax.swing.*;
import java.awt.*;
import java.awt.image.BufferedImage;
import java.io.IOException;
import java.util.Objects;

public class CardBackView extends JPanel {
    private static BufferedImage cardBackImage;

    static {
        try {
            cardBackImage = ImageIO.read(Objects.requireNonNull(CardBackView.class.getResource("/cardback.jpg")));
        } catch (IOException | IllegalArgumentException e) {
            e.printStackTrace();
            System.err.println("Failed to load card back image: " + e.getMessage());
            cardBackImage = null;
        }
    }

    public CardBackView() {
        setPreferredSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT));
        setMaximumSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT));
    }

    protected void paintComponent(Graphics g) {
        super.paintComponent(g);
        if (cardBackImage != null) {
            g.drawImage(cardBackImage, 0, 0, Constants.CARD_WIDTH, Constants.CARD_HEIGHT, this);
        } else {
            g.setColor(new Color(96, 151, 234));
            g.fillRect(0, 0, Constants.CARD_WIDTH, Constants.CARD_HEIGHT);
            g.setColor(Color.BLACK);
            g.drawRect(0, 0, Constants.CARD_WIDTH - 1, Constants.CARD_HEIGHT - 1);
        }
    }
}
