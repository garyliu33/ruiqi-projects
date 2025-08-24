import java.io.*;

public class Defender extends Player {
    private boolean usedCauldron;

    public Defender(PlayerType type, Input input) {
        super(type, input);
        usedCauldron = false;
    }

    public Played playCard() throws IOException {
        Card card = chooseCard();
        if (card == null) {
            return Played.FAILED;
        } else if (card.isAction()) {
            return Played.USED_ACTION;
        }

        int wall = chooseWall();
        if (wall == 0) {
            return Played.FAILED;
        }

        Played played = Table.getInstance().playCard(card, wall, false);

        if (played == Played.SUCCEEDED) {
            hand.remove(card);
            return Played.SUCCEEDED;
        } else if (played == Played.NO_SPACE) {
            displayln(Prompts.NO_SPACE);
        }
        return Played.FAILED;
    }

    private Card chooseCard() throws IOException {
        clearInput();
        if (Table.getInstance().getCauldronCount() > 0 && !usedCauldron) {
            display("Which card (c for cauldron)? ", "GET_INPUT");
        } else {
            display("Which card? ", "GET_INPUT");
        }
        String c = input.readLine();
        if (c.equalsIgnoreCase("c") && !usedCauldron) {
            if (cauldron()) {
                return Card.ACTION;
            }
            return null;
        } else if (c.equalsIgnoreCase("c")) {
            displayln("You already used a cauldron this turn");
            return null;
        }

        if (!Card.isValid(c)) {
            displayln(Prompts.INVALID_CARD);
            return null;
        }

        Card card = new Card(c);
        if (!hand.contains(card)) {
            displayln(Prompts.NO_CARD);
            return null;
        }

        usedCauldron = false;
        return card;
    }

    private boolean cauldron() throws IOException {
        if (Table.getInstance().getCauldronCount() > 0) {
            int wall = chooseWall();
            if (wall != 0) {
                if (Table.getInstance().cauldron(wall)) {
                    usedCauldron = true;
                    return true;
                } else {
                    displayln("Nothing to cauldron");
                }
            }
        } else {
            displayln("You have no more cauldrons");
        }
        return false;
    }
}
