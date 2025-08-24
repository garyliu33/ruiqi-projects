import javax.swing.*;
import java.awt.*;

public class HandView extends JPanel {
    private final Player player;
    private CardContainer selectedCard = null;

    public HandView(Player player) {
        this.player = player;
        setLayout(new FlowLayout(FlowLayout.CENTER, 0, 0));
        setMaximumSize(new Dimension(Constants.WINDOW_WIDTH, Constants.CARD_HEIGHT));
        update();
    }

    public void update() {
        removeAll();
        boolean isAttacker = player.getPlayerType() == PlayerType.ATTACKER;
        for (Card card : player.getHand().getCards()) {
            add(new CardContainer(card, !isAttacker, this));
        }
        if (isAttacker) {
            add(new CardContainer(Card.RETREAT, false, this));
        } else if (!player.hasUsedCauldron()) {
            for (int i = 0; i < player.getCauldronCount(); i++) {
                add(new CardContainer(Card.CAULDRON, true, this));
            }
        }
        revalidate();
        repaint();
    }

    public void notifyCardClicked(CardContainer clickedCard) {
        if (selectedCard != null && selectedCard != clickedCard) {
            selectedCard.unPop();
        } else if (selectedCard == clickedCard) {
            unselectCard();
            return;
        }
        selectedCard = clickedCard;
    }

    public Card getSelectedCard() {
        return selectedCard == null ? null : selectedCard.getCard();
    }

    public void unselectCard() {
        selectedCard = null;
    }
}
