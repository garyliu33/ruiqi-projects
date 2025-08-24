import javax.swing.*;
import java.awt.*;

public class DeckView extends JPanel {
    public DeckView(int deckSize) {
        setLayout(new BoxLayout(this, BoxLayout.Y_AXIS));
        JLabel invisLabel = new JLabel("deck", SwingConstants.CENTER);
        invisLabel.setAlignmentX(Component.CENTER_ALIGNMENT);
        invisLabel.setForeground(new Color(0, 0, 0, 0));
        invisLabel.setFont(new Font("Arial", Font.BOLD, Constants.CARD_FONT_SIZE));

        JLabel label = new JLabel(deckSize + "", SwingConstants.CENTER);
        label.setAlignmentX(Component.CENTER_ALIGNMENT);
        label.setFont(new Font("Arial", Font.BOLD, Constants.CARD_FONT_SIZE));


        JPanel cardBackView = new CardBackView();
        cardBackView.setAlignmentX(Component.CENTER_ALIGNMENT);

        add(invisLabel);
        add(cardBackView);
        add(label);
    }
}
