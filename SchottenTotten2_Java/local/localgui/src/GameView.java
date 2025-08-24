import javax.swing.*;
import java.awt.*;
import java.util.function.Consumer;

public class GameView extends JPanel {
    private final Game game;
    private final TableView tableView;
    private final HandView attackerHandView;
    private final HandView defenderHandView;

    public GameView(Game game, Consumer<Wall> onWallClicked) {
        this.game = game;
        setLayout(new BoxLayout(this, BoxLayout.Y_AXIS));
        attackerHandView = new HandView(game.getAttacker());
        defenderHandView = new HandView(game.getDefender());
        tableView = new TableView(game.getBoard(), game.getDeck(), game.getDiscard(), onWallClicked);

        add(defenderHandView);
        add(Box.createVerticalGlue());
        add(tableView);
        add(Box.createVerticalGlue());
        add(attackerHandView);
    }

    public void update() {
        attackerHandView.update();
        defenderHandView.update();
        tableView.update();
    }

    public void displayWinner(Winner winner) {
        String message = switch (winner) {
            case ATTACKER -> "Attacker wins!";
            case DEFENDER -> "Defender wins!";
            case NONE -> null;
        };
        JOptionPane.showMessageDialog(this, message, "Game Over", JOptionPane.INFORMATION_MESSAGE);
        update();
    }

    public Card getSelectedAttackerCard() {
        return attackerHandView.getSelectedCard();
    }

    public Card getSelectedDefenderCard() {
        return defenderHandView.getSelectedCard();
    }

    public void unselectAttackerCard() {
        attackerHandView.unselectCard();
    }

    public void unselectDefenderCard() {
        defenderHandView.unselectCard();
    }
}
