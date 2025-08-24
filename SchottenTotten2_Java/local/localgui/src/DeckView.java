import javax.swing.*;
import java.awt.*;

public class DeckView extends JPanel {
    private final Deck deck;

    public DeckView(Deck deck) {
        this.deck = deck;
        setLayout(new BoxLayout(this, BoxLayout.Y_AXIS));
        update();
    }

    public void update() {
        removeAll();
        JLabel invisLabel = new JLabel("deck", SwingConstants.CENTER);
        invisLabel.setAlignmentX(Component.CENTER_ALIGNMENT);
        invisLabel.setForeground(new Color(0, 0, 0, 0));

        JLabel label = new JLabel(deck.size() + "", SwingConstants.CENTER);
        label.setAlignmentX(Component.CENTER_ALIGNMENT);

        JPanel cardBackView = new CardBackView();
        cardBackView.setAlignmentX(Component.CENTER_ALIGNMENT);

        add(invisLabel);
        add(cardBackView);
        add(label);

        revalidate();
        repaint();
    }
}
