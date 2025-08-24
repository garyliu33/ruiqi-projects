import javax.swing.*;
import java.awt.*;

public class CardContainer extends JPanel {
    private final CardView cardView;
    private boolean popped;
    private final HandView parent;

    public CardContainer(Card card, HandView parent) {
        this.parent = parent;
        setLayout(null);
        setPreferredSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT + Constants.POP_OFFSET));
        setOpaque(false);
        this.cardView = new CardView(card, this::mouseClicked);
        cardView.setBounds(0, Constants.POP_OFFSET, Constants.CARD_WIDTH, Constants.CARD_HEIGHT);
        add(cardView);
        popped = false;
    }

    public void mouseClicked() {
        parent.notifyCardClicked(this);
        popped = !popped;
        updatePosition();
    }

    public void unPop() {
        popped = false;
        parent.unselectCard();
        updatePosition();
    }

    public void updatePosition() {
        cardView.setLocation(0, popped ? 0 : Constants.POP_OFFSET);
    }

    public Card getCard() {
        return cardView.getCard();
    }
}
