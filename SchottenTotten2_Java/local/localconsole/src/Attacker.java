import java.util.*;

public class Attacker extends Player {
    public Attacker() {
        super();
    }

    public boolean playCard() {
        Scanner scan = new Scanner(System.in);
        Card card = chooseCard(scan);
        if (card == null) {
            return false;
        }

        int wall = chooseWall(scan);
        if (wall == 0) {
            return false;
        }

        if (Board.getInstance().playCard(card, wall, true)) {
            hand.remove(card);
            return true;
        }
        return false;
    }

    private Card chooseCard(Scanner scan) {
        System.out.print("Which card (r for retreat)? ");
        String c = scan.nextLine();
        if (c.equalsIgnoreCase("r")) {
            retreat(scan);
            return null;
        }

        if (!Card.isValid(c)) {
            System.out.println("invalid move");
            System.out.println("your opponent smacks you");
            return null;
        }

        Card card = new Card(c);
        if (!hand.contains(card)) {
            System.out.println("you don't have that card");
            System.out.println("you clearly need glasses");
            return null;
        }

        return card;
    }

    private void retreat(Scanner scan) {
        int wall = chooseWall(scan);
        if (wall != 0) {
            Board.getInstance().retreat(wall);
        }
    }
}
