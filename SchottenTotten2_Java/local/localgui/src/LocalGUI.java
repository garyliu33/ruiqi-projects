import javax.swing.*;

public class LocalGUI {
    public static void main(String[] args) {
        JFrame mainFrame = new JFrame("Schotten Totten 2 (Local)");
        mainFrame.setSize(Constants.WINDOW_WIDTH, Constants.WINDOW_HEIGHT);
        mainFrame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

        GameController gameController = new GameController(new Game(new Attacker(), new Defender(), new Board(), new Deck(), new Discard()));
        gameController.runGame();
        mainFrame.add(gameController.getGameView());
        mainFrame.setVisible(true);
    }
}
