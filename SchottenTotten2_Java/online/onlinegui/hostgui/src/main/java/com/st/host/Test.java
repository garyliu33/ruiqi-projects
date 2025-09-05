package com.st.host;

import java.awt.FlowLayout;

import javax.swing.JFrame;

import com.st.common.Card;
import com.st.common.CardView;
import com.st.common.Constants;

public class Test {
    public static void main(String[] args) {
        JFrame mainFrame = new JFrame("Schotten Totten 2 (host)");
        mainFrame.setSize(Constants.WINDOW_WIDTH, Constants.WINDOW_HEIGHT);
        mainFrame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        mainFrame.setVisible(true);

        mainFrame.setLayout(new FlowLayout());
        mainFrame.add(new CardView(Card.CAULDRON, false));
        mainFrame.add(new CardView(Card.RETREAT, false));
        mainFrame.add(new CardBackView());
        mainFrame.revalidate();
        mainFrame.repaint();
    }
}
