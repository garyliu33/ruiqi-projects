public class Local {
    private static final int delay = 5000; // delay between displaying hands
    public static final boolean useEmojis = true; // change to false if emojis don't load

    public static void main(String[] args) {
        Board board = Board.getInstance();
        Player attacker = new Attacker();
        Player defender = new Defender();
        board.setup(attacker, defender);
        displayInstructions();

        while (true) {
            board.display();
            delay();
            attacker.takeTurn();
            board.declareControl();
            attacker.draw();

            int won = board.won();
            if (won != Constants.noWinner) {
                board.display();
                displayWinner(won);
                break;
            }

            board.display();
            delay();
            defender.takeTurn();
            defender.draw();
            board.declareControl();
        }
    }

    private static void displayWinner(int won) {
        if (won == Constants.attackerWins) {
            System.out.println("Attacker wins");
        } else if (won == Constants.defenderWins) {
            System.out.println("Defender wins");
        } else {
            System.out.println("Game not over yet");
        }
    }

    private static void delay() {
        try {
            Thread.sleep(delay);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
    }

    private static void displayInstructions() {
        if (Local.useEmojis) {
            System.out.println("When playing a card, type either the name, color, or copy paste emoji of the suit followed by the number (no space).");
            System.out.println("Names: heart, diamond, star, clover, flower");
            System.out.println("Colors: red, blue, yellow, green, pink");
            System.out.println("Emojis: ♥️ \uD83D\uDC8E ⭐️ \uD83C\uDF40 \uD83C\uDF38");
        } else {
            System.out.println("When playing a card, type it exactly as it is displayed by the game.");
        }
        System.out.println("Single digit numbers must have a 0 in front of them");
        System.out.println();
    }
}