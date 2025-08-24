import javax.swing.*;
import java.awt.*;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;

public class CardContainer extends JPanel {
    private final CardView cardView;
    private boolean popped;
    private final boolean popDown;
    private static final int POP_OFFSET = 15;
    private final HandView parent;

    public CardContainer(Card card, boolean popDown, HandView parent) {
        this.popDown = popDown;
        this.parent = parent;
        setLayout(null);
        setPreferredSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT + POP_OFFSET));
        setOpaque(false);
        this.cardView = new CardView(card, this::mouseClicked);
        cardView.setBounds(0, popDown ? 0 : POP_OFFSET, Constants.CARD_WIDTH, Constants.CARD_HEIGHT);
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
        if (popped) {
            cardView.setLocation(0, popDown ? POP_OFFSET : 0);
        } else {
            cardView.setLocation(0, popDown ? 0 : POP_OFFSET);
        }
    }

    public Card getCard() {
        return cardView.getCard();
    }
}
