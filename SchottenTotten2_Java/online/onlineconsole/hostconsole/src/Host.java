import java.util.*;
import java.io.*;
import java.net.*;

public class Host {
    public static boolean useEmojis;
    private static Player attacker;
    private static Player defender;

    public static void main(String[] args) {
        try (ServerSocket serverSocket = new ServerSocket(Integer.parseInt(args[0]))) {
            Display.toHostln("Waiting for client to connect...");
            try (Socket socket = serverSocket.accept()) {
                Display.toHostln("Connected!\n");

                BufferedReader cIn = new BufferedReader(new InputStreamReader(socket.getInputStream()));
                PrintWriter clientOut = new PrintWriter(socket.getOutputStream(), true);
                BufferedReader hIn = new BufferedReader(new InputStreamReader(System.in));
                Display.setClientOut(clientOut);

                Input clientIn = new Input(cIn);
                Input hostIn = new Input(hIn);

                useEmojis = emojiCheck(hostIn, clientIn);
                Display.toBothln(instructions());

                Display.toClient("Host is choosing role");
                Role hostRole = chooseRole(hostIn);
                boolean playAgain = runGame(clientIn, hostIn, hostRole);
                while (playAgain) {
                    Display.toClient("Host is choosing role");
                    hostRole = chooseRole(hostRole, hostIn);
                    playAgain = runGame(clientIn, hostIn, hostRole);
                }

                Display.toBothln(Prompts.GAME_OVER, "END_PROGRAM");
            }
        } catch (NumberFormatException e) {
            System.err.println("Invalid port number.");
        } catch (IOException e) {
            System.err.println("Error in server connection: " + e.getMessage());
        } catch (Exception e) {
            System.err.println("An unexpected error occurred: " + e.getMessage());
        }
    }

    private static boolean runGame(Input clientIn, Input hostIn, Role hostRole) throws IOException {
        if (hostRole == Role.ATTACKER) {
            attacker = new Attacker(Player.PlayerType.HOST, hostIn);
            defender = new Defender(Player.PlayerType.CLIENT, clientIn);
            Display.toHostln("\nYou are the ATTACKER.");
            Display.toClient("\nYou are the DEFENDER.");
        } else {
            attacker = new Attacker(Player.PlayerType.CLIENT, clientIn);
            defender = new Defender(Player.PlayerType.HOST, hostIn);
            Display.toClient("\nYou are the ATTACKER.");
            Display.toHostln("\nYou are the DEFENDER.");
        }

        Table table = Table.getInstance();
        table.setup(attacker, defender);

        while (true) {
            Display.toBothln(table.toString());
            displayHands();
            if (displayWinner(table, false)) {
                break;
            }
            attacker.takeTurn();
            table.declareControl();
            attacker.draw();

            Display.toBothln(table.toString());
            displayHands();
            if (displayWinner(table, true)) {
                break;
            }
            defender.takeTurn();
            defender.draw();
            table.declareControl();
        }

        return playAgain(hostIn);
    }

    private static boolean playAgain(Input hostIn) throws IOException {
        while (true) {
            Display.toHost(Prompts.REMATCH);
            hostIn.clear();
            String str = hostIn.readLine().trim();
            if (str.equalsIgnoreCase("y")) {
                return true;
            } else if (str.equalsIgnoreCase("n")) {
                return false;
            } else {
                Display.toHostln(Prompts.TRY_AGAIN);
            }
        }
    }

    private static Role chooseRole(Input hostIn) throws IOException {
        while (true) {
            Display.toHost("Which role (attacker/defender/random)? ");
            hostIn.clear();
            String role = hostIn.readLine().trim().toLowerCase();
            if ("attacker".startsWith(role)) {
                return Role.ATTACKER;
            } else if ("defender".startsWith(role)) {
                return Role.DEFENDER;
            } else if ("random".startsWith(role)) {
                int i = (int)(2 * Math.random());
                return i == 0 ? Role.ATTACKER : Role.DEFENDER;
            } else {
                Display.toHostln(Prompts.TRY_AGAIN);
            }
        }
    }

    private static Role chooseRole(Role prevHostRole, Input hostIn) throws IOException {
        while (true) {
            Display.toHost("Which role (attacker/defender/random/swap)? ");
            hostIn.clear();
            String role = hostIn.readLine().trim().toLowerCase();
            if ("attacker".startsWith(role)) {
                return Role.ATTACKER;
            } else if ("defender".startsWith(role)) {
                return Role.DEFENDER;
            } else if ("random".startsWith(role)) {
                int i = (int) (2 * Math.random());
                return i == 0 ? Role.ATTACKER : Role.DEFENDER;
            } else if ("swap".startsWith(role)) {
                return prevHostRole.other();
            } else {
                Display.toHostln(Prompts.TRY_AGAIN);
            }
        }
    }

    private static boolean emojiCheck(Input hostIn, Input clientIn) throws IOException {
        return hostEmojiCheck(hostIn) && clientEmojiCheck(clientIn);
    }

    private static boolean hostEmojiCheck(Input hostIn) throws IOException {
        while (true) {
            Display.toClient("Checking host emojis");
            Display.toHost(Prompts.EMOJI_CHECK);
            hostIn.clear();
            String str = hostIn.readLine().trim();
            if (str.equalsIgnoreCase("y")) {
                return true;
            } else if (str.equalsIgnoreCase("n")) {
                return false;
            } else {
                Display.toHostln(Prompts.TRY_AGAIN);
            }
        }
    }

    private static boolean clientEmojiCheck(Input clientIn) throws IOException {
        while (true) {
            Display.toHostln("Checking client emojis");
            Display.toClient(Prompts.EMOJI_CHECK, "GET_INPUT");
            String str = clientIn.readLine().trim();
            if (str.equalsIgnoreCase("y")) {
                return true;
            } else if (str.equalsIgnoreCase("n")) {
                return false;
            } else {
                Display.toClient(Prompts.TRY_AGAIN);
            }
        }
    }

    private static String instructions() {
        StringBuilder str = new StringBuilder();
        if (useEmojis) {
            str.append("\nWhen playing a card, type either the name, color, or copy paste emoji of the suit followed by the number (no space).");
            str.append("\nNames: ").append(Color.listOf(Color.ColorType.NAME));
            str.append("\nColors: ").append(Color.listOf(Color.ColorType.COLOR));
            str.append("\nEmojis: ").append(Color.listOf(Color.ColorType.EMOJI));
        } else {
            str.append("\nWhen playing a card, type it as it is displayed by the system");
            str.append("\nSuits: ").append(Color.listOf(Color.ColorType.FRUIT));
        }
        str.append("\nSingle digit numbers must have a 0 in front of them.\n");
        str.append("\nType \"quit\" at any time to exit the game.\n");
        return str.toString();
    }

    private static boolean displayWinner(Table table, boolean checkDeck) {
        return switch(table.won(checkDeck)) {
            case Winner.ATTACKER -> {
                attacker.displayln(Prompts.WIN);
                defender.displayln(Prompts.LOSE);
                yield true;
            }
            case Winner.DEFENDER -> {
                defender.displayln(Prompts.WIN);
                attacker.displayln(Prompts.LOSE);
                yield true;
            }
            default -> false;
        };
    }

    public static void displayHands() {
        attacker.displayHand();
        defender.displayHand();
    }

    private enum Role {
        ATTACKER, DEFENDER;

        public Role other() {
            return this == ATTACKER ? DEFENDER : ATTACKER;
        }
    }

    private static class Prompts {
        static final String REMATCH = "Rematch (y/n)? ";
        static final String TRY_AGAIN = "Let's try that again.";
        static final String EMOJI_CHECK = "Here are the emojis used in the game:\n" +
                Color.listOf(Color.ColorType.EMOJI) + ", " + Constants.CAULDRON +
                "\nCan you see them (y/n)? ";
        static final String GAME_OVER = "GAME OVER\nTHANKS FOR PLAYING";
        static final String WIN = "\nYOU WIN\n";
        static final String LOSE = "\nYOU LOSE\n";
    }
}
