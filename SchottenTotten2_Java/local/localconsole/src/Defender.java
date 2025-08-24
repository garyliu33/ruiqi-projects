import java.util.*;

public class Defender extends Player {
    private boolean usedCauldron;

    public Defender() {
        super();
        usedCauldron = false;
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

        if (Board.getInstance().playCard(card, wall, false)) {
            hand.remove(card);
            return true;
        }
        return false;
    }

    private Card chooseCard(Scanner scan) {
        if (Board.getInstance().getCauldronCount() > 0 && !usedCauldron) {
            System.out.print("Which card (c for cauldron)? ");
        } else {
            System.out.print("Which card? ");
        }
        String c = scan.nextLine();
        if (c.equalsIgnoreCase("c") && !usedCauldron) {
            cauldron(scan);
            return null;
        } else if (c.equalsIgnoreCase("c")) {
            System.out.println("you already tried that this turn");
            System.out.println("you cheater");
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

        usedCauldron = false;
        return card;
    }

    private void cauldron(Scanner scan) {
        if (Board.getInstance().getCauldronCount() > 0) {
            int wall = chooseWall(scan);
            if (wall != 0) {
                if (Board.getInstance().cauldron(wall)) {
                    usedCauldron = true;
                } else {
                    System.out.println("nothing to cauldron");
                    System.out.println("thanks for watering the plants with hot oil i guess");
                    System.out.println("jk have your cauldron back");
                }
            }
        } else {
            System.out.println("you have no more cauldrons");
            System.out.println("cry about it");
        }
    }
}
