import javax.swing.*;
import java.awt.*;
import java.util.function.Consumer;

public class BoardView extends JPanel {
    private final Board board;
    private final Consumer<Wall> onWallClicked;

    public BoardView(Board board, Consumer<Wall> onWallClicked) {
        this.board = board;
        this.onWallClicked = onWallClicked;
        int hgap = 15;
        setLayout(new FlowLayout(FlowLayout.CENTER, hgap, 0));
        setMaximumSize(new Dimension(Constants.NUM_WALLS * WallView.WALL_WIDTH + (Constants.NUM_WALLS - 1) * hgap, WallView.OVERALL_HEIGHT));
        update();
    }

    public void update() {
        removeAll();
        for (Wall wall : board.getWalls()) {
            add(new WallView(wall, onWallClicked));
        }
        revalidate();
        repaint();
    }
}
