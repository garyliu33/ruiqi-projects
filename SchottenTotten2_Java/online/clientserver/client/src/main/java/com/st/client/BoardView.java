package com.st.client;

import java.awt.Dimension;
import java.awt.FlowLayout;
import java.util.function.Consumer;

import javax.swing.JPanel;

import com.st.common.Card;
import com.st.common.Constants;
import com.st.common.Wall;

public class BoardView extends JPanel {
    public BoardView(Wall[] walls, Consumer<Wall> onWallClicked, boolean isHostAttacker, Card lastPlayedCard) {
        int hgap = 15;
        setLayout(new FlowLayout(FlowLayout.CENTER, hgap, 0));
        setMaximumSize(new Dimension(Constants.NUM_WALLS * Constants.WALL_WIDTH + (Constants.NUM_WALLS - 1) * hgap, Constants.WALL_OVERALL_HEIGHT));
        for (Wall wall : walls) {
            add(new WallView(wall, onWallClicked, isHostAttacker, lastPlayedCard));
        }
    }
}