import java.util.*;

public abstract class Player {
    protected Set<Card> hand;

    public Player() {
        hand = new TreeSet<>();
    }

    public void takeTurn() {
        displayHand();
        boolean played = playCard();
        while (!played) {
            displayHand();
            played = playCard();
        }
    }

    protected int chooseWall(Scanner scan) {
        System.out.print("Which wall (0 to cancel)? ");
        String w = scan.nextLine();
        if (!isInteger(w)) {
            System.out.println("that's not a wall");
            System.out.println("watch where you send your troops");
            return 0;
        }

        int wall = Integer.parseInt(w);
        if (wall == 0) {
            System.out.println("you have commitment issues");
            return 0;
        } else if (wall < 0 || wall > Constants.numWalls) {
            System.out.println("wall out of range");
            System.out.println("bros sending troops to narnia");
            return 0;
        }

        return wall;
    }

    private boolean isInteger(String str) {
        if (str == null || str.isEmpty()) {
            return false;
        }
        try {
            Integer.parseInt(str);
            return true;
        } catch (NumberFormatException e) {
            return false;
        }
    }

    public void draw() {
        Card card = Deck.getInstance().pop();
        if (card != null) {
            hand.add(card);
        }
    }

    public abstract boolean playCard();

    public void displayHand() {
        for (Card card : hand) {
            System.out.print(card + " ");
        }
        System.out.println();
    }
}