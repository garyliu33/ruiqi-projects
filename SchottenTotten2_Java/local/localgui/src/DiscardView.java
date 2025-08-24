import javax.swing.*;
import java.awt.*;
import java.util.List;
import java.util.Map;

public class DiscardView extends JPanel {
    private final Discard discard;

    public DiscardView(Discard discard) {
        this.discard = discard;
        setLayout(new FlowLayout(FlowLayout.CENTER, 10, 0));
        update();
    }

    public void update() {
        removeAll();
        Map<CardColor, List<Card>> cardsByColor = discard.getCardsByColor();
        for (CardColor color : cardsByColor.keySet()) {
            List<Card> cards = cardsByColor.get(color);
            if (!cards.isEmpty()) {
                JPanel column = new JPanel();
                column.setLayout(null);
                column.setPreferredSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT + (Constants.VALUES.size() - 1) * Constants.OVERLAP));
                for (int i = cards.size() - 1; i >= 0; i--) {
                    Card card = cards.get(i);
                    CardView cardView = new CardView(card);
                    cardView.setBounds(0, card.getValue() * Constants.OVERLAP, Constants.CARD_WIDTH, Constants.CARD_HEIGHT);
                    column.add(cardView);
                }
                add(column);
            }
        }
    }
}
