import javax.swing.*;
import java.awt.*;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;

public class CardView extends JPanel {
    private final Card card;
    private boolean hovered;

    public CardView(Card card) {
        this.card = card;
        hovered = false;
        setPreferredSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT));
        setBorder(BorderFactory.createLineBorder(Color.BLACK));
    }

    public CardView(Card card, Runnable onClick) {
        this.card = card;
        hovered = false;
        setPreferredSize(new Dimension(Constants.CARD_WIDTH, Constants.CARD_HEIGHT));
        setBorder(BorderFactory.createLineBorder(Color.BLACK));

        addMouseListener(new MouseAdapter() {
            @Override
            public void mouseClicked(MouseEvent e) {
                onClick.run();
            }

            @Override
            public void mouseEntered(MouseEvent e) {
                hovered = true;
                updateBorder();
            }

            @Override
            public void mouseExited(MouseEvent e) {
                hovered = false;
                updateBorder();
            }
        });
    }

    protected void paintComponent(Graphics g) {
        super.paintComponent(g);
        Graphics2D g2 = (Graphics2D) g.create();

        if (card != null) {
            g2.setColor(Color.WHITE);
            g2.fillRect(0, 0, getWidth(), getHeight());
            g2.setColor(Color.BLACK);
            g2.drawRect(0, 0, getWidth() - 1, getHeight() - 1);

            g2.setColor(card.getColor().getDisplayColor());
            g2.setFont(new Font("Arial", Font.BOLD, 18));
            FontMetrics fm = g2.getFontMetrics();
            int padding = 5;
            if (card.equals(Card.RETREAT)) {
                g2.drawString("RETR", padding, fm.getAscent() + padding);
                g2.drawString("EAT", padding, fm.getAscent() + padding + fm.getHeight());
            } else if (card.equals(Card.CAULDRON)) {
                g2.drawString("CAUL", padding, fm.getAscent() + padding);
                g2.drawString("DRON", padding, fm.getAscent() + padding + fm.getHeight());
            } else {
                String text = card.getValue() + "";
                g2.drawString(text, padding, fm.getAscent() + padding);
                g2.translate(getWidth(), getHeight());
                g2.rotate(Math.PI);
                g2.drawString(text, padding, fm.getAscent() + padding);
                g2.dispose();
            }
        }
    }

    private void updateBorder() {
        if (hovered) {
            setBorder(BorderFactory.createLineBorder(Color.GREEN, 3));
        } else {
            setBorder(BorderFactory.createLineBorder(Color.BLACK));
        }
        repaint();
    }

    public Card getCard() {
        return card;
    }
}