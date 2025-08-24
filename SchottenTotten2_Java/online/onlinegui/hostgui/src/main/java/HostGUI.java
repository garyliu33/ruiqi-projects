import javax.swing.*;
import java.awt.*;
import java.awt.event.ComponentAdapter;
import java.awt.event.ComponentEvent;
import java.io.IOException;
import java.net.*;
import java.util.Enumeration;

public class HostGUI {
    private static GameController gameController;
    private static JFrame mainFrame;
    private static Role hostRole;

    public static void main(String[] args) throws IOException {
        mainFrame = new JFrame("Schotten Totten 2 (host)");
        mainFrame.setSize(Constants.WINDOW_WIDTH, Constants.WINDOW_HEIGHT);
        mainFrame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        mainFrame.setVisible(true);

        ServerSocket serverSocket = new ServerSocket(12345);
        JLabel ipLabel = new JLabel("Host IP: " + getLocalIPv4Address());
        ipLabel.setHorizontalAlignment(SwingConstants.CENTER);
        mainFrame.add(ipLabel, BorderLayout.CENTER);
        mainFrame.revalidate();
        mainFrame.repaint();
        Network network = new Network(serverSocket);
        mainFrame.getContentPane().removeAll();
        mainFrame.revalidate();
        mainFrame.repaint();

        hostRole = chooseHostRole(true);
        gameController = new GameController(new Game(new Player(), new Player(), new Board(), new Deck(), new Discard()), hostRole, network);
        mainFrame.addComponentListener(new ComponentAdapter() {
            @Override
            public void componentResized(ComponentEvent e) {
                Constants.resize(mainFrame.getWidth(), mainFrame.getHeight());
                gameController.updateGameView();
                displayGameState();
            }
        });

        while (true) {
            gameController.startGame();
            while (!gameController.gameOver()) {
                try {
                    Thread.sleep(50);
                } catch (InterruptedException e) {
                    throw new RuntimeException(e);
                }
            }

            if (gameController.playAgain()) {
                hostRole = chooseHostRole(false);
                gameController = new GameController(new Game(new Player(), new Player(), new Board(), new Deck(), new Discard()), hostRole, network);
            } else {
                break;
            }
        }
    }

    public static void displayGameState() {
        mainFrame.getContentPane().removeAll();
        mainFrame.add(gameController.getGameView());
        mainFrame.revalidate();
        mainFrame.repaint();
    }

    public static Role chooseHostRole(boolean isFirstGame) {
        if (isFirstGame) {
            String[] options = {"Attacker", "Defender", "Random"};
            while (true) {
                int choice = JOptionPane.showOptionDialog(
                        mainFrame,
                        "Choose your role",
                        "Select Role",
                        JOptionPane.DEFAULT_OPTION,
                        JOptionPane.QUESTION_MESSAGE,
                        null,
                        options,
                        options[2]
                );

                if (choice == JOptionPane.CLOSED_OPTION) {
                    int confirm = JOptionPane.showConfirmDialog(mainFrame, "Exit game?", "Confirm Exit", JOptionPane.YES_NO_OPTION);
                    if (confirm == JOptionPane.YES_OPTION) {
                        System.exit(0);
                    }
                    continue;
                }

                switch (choice) {
                    case 0: return Role.ATTACKER;
                    case 1: return Role.DEFENDER;
                    case 2: return Math.random() < 0.5 ? Role.ATTACKER : Role.DEFENDER;
                }
            }
        } else {
            String[] options = {"Attacker", "Defender", "Random", "Swap"};
            while (true) {
                int choice = JOptionPane.showOptionDialog(
                        mainFrame,
                        "Choose your role",
                        "Select Role",
                        JOptionPane.DEFAULT_OPTION,
                        JOptionPane.QUESTION_MESSAGE,
                        null,
                        options,
                        options[3]
                );

                if (choice == JOptionPane.CLOSED_OPTION) {
                    int confirm = JOptionPane.showConfirmDialog(mainFrame, "Exit game?", "Confirm Exit", JOptionPane.YES_NO_OPTION);
                    if (confirm == JOptionPane.YES_OPTION) {
                        System.exit(0);
                    }
                    continue;
                }

                switch (choice) {
                    case 0: return Role.ATTACKER;
                    case 1: return Role.DEFENDER;
                    case 2: return Math.random() < 0.5 ? Role.ATTACKER : Role.DEFENDER;
                    case 3: return hostRole == Role.ATTACKER ? Role.DEFENDER : Role.ATTACKER;
                }
            }
        }
    }

    private static String getLocalIPv4Address() {
        try {
            Enumeration<NetworkInterface> interfaces = NetworkInterface.getNetworkInterfaces();
            while (interfaces.hasMoreElements()) {
                NetworkInterface ni = interfaces.nextElement();
                if (!ni.isUp() || ni.isLoopback() || ni.isVirtual()) continue;

                Enumeration<InetAddress> addresses = ni.getInetAddresses();
                while (addresses.hasMoreElements()) {
                    InetAddress addr = addresses.nextElement();
                    if (addr instanceof Inet4Address && !addr.isLoopbackAddress()) {
                        String ip = addr.getHostAddress();
                        if (ip.startsWith("192.168.") || ip.startsWith("10.") || ip.matches("172\\.(1[6-9]|2[0-9]|3[0-1])\\..*")) {
                            return ip;
                        }
                    }
                }
            }
        } catch (SocketException e) {
            e.printStackTrace();
        }
        return "Unable to determine local IP";
    }

    public static void clientDisconnected() {
        mainFrame.getContentPane().removeAll();
        JLabel label = new JLabel("Waiting for client to reconnect...");
        label.setHorizontalAlignment(SwingConstants.CENTER);
        mainFrame.add(label, BorderLayout.CENTER);
        mainFrame.revalidate();
        mainFrame.repaint();
    }

    public static boolean showRematchDialog(Winner winner) {
        Object[] options = {"Yes", "No"};
        int result = JOptionPane.showOptionDialog(
                mainFrame,
                (winner == Winner.ATTACKER ? "Attacker" : "Defender") + " wins!\nRematch?",
                "Game Over",
                JOptionPane.DEFAULT_OPTION,
                JOptionPane.INFORMATION_MESSAGE,
                null,
                options,
                options[0]
        );
        return result == 0;
    }
}
