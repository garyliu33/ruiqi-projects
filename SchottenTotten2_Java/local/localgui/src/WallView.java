import javax.swing.*;
import java.awt.*;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;
import java.util.List;
import java.util.function.Consumer;

public class WallView extends JPanel {
    private final Wall wall;
    public static final int OVERALL_HEIGHT = Constants.WINDOW_HEIGHT - 3 * Constants.CARD_HEIGHT;
    public static final int WALL_WIDTH = Constants.CARD_WIDTH;
    public static final int WALL_HEIGHT = 50;

    public WallView(Wall wall, Consumer<Wall> onWallClicked) {
        this.wall = wall;
        setLayout(null);
        setPreferredSize(new Dimension(Constants.CARD_WIDTH, OVERALL_HEIGHT));
        setOpaque(true);

        List<Card> topCards = wall.getDefenderCards();
        for (int i = topCards.size() - 1; i >= 0; i--) {
            CardView cardView = new CardView(topCards.get(i));
            cardView.setBounds(0, (OVERALL_HEIGHT - WALL_HEIGHT) / 2 - i * Constants.OVERLAP - Constants.OVERLAP / 2 - Constants.CARD_HEIGHT, Constants.CARD_WIDTH, Constants.CARD_HEIGHT);
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
            label.setBounds(0, (OVERALL_HEIGHT - WALL_HEIGHT) / 2, WALL_WIDTH, WALL_HEIGHT);
            add(label);
        }

        List<Card> bottomCards = wall.getAttackerCards();
        for (int i = bottomCards.size() - 1; i >= 0; i--) {
            CardView cardView = new CardView(bottomCards.get(i));
            cardView.setBounds(0, (OVERALL_HEIGHT + WALL_HEIGHT) / 2 + i * Constants.OVERLAP + Constants.OVERLAP / 2, Constants.CARD_WIDTH, Constants.CARD_HEIGHT);
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
