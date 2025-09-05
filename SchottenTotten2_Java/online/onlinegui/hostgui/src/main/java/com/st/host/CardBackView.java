package com.st.host;

import java.awt.Color;
import java.awt.Dimension;
import java.awt.Graphics;
import java.awt.Graphics2D;
import java.awt.image.BufferedImage;
import java.io.IOException;
import java.util.Objects;

import javax.imageio.ImageIO;
import javax.swing.BorderFactory;
import javax.swing.JPanel;

import com.st.common.Constants;

public class CardBackView extends JPanel {
    private static BufferedImage cardBackImage;

    static {
        try {
            cardBackImage = ImageIO.read(Objects.requireNonNull(CardBackView.class.getResource("/cardback.jpg")));
        } catch (IOException | IllegalArgumentException e) {
            cardBackImage = null;
            e.printStackTrace();
        }
    }

    public CardBackView() {
        setPreferredSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT));
        setMaximumSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT));
        setBorder(BorderFactory.createLineBorder(Color.BLACK));
    }

    protected void paintComponent(Graphics g) {
        super.paintComponent(g);
        Graphics2D g2 = (Graphics2D) g.create();
        if (cardBackImage != null) {
            g2.drawImage(cardBackImage, 0, 0, Constants.CARD_WIDTH, Constants.CARD_HEIGHT, this);
            g2.setColor(Color.BLACK);
            g2.drawRect(0, 0, Constants.CARD_WIDTH - 1, Constants.CARD_HEIGHT - 1);
        } else {
            g2.setColor(new Color(96, 151, 234));
            g2.fillRect(0, 0, Constants.CARD_WIDTH, Constants.CARD_HEIGHT);
        }
    }
}
