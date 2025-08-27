package com.st.client;

import com.google.gson.Gson;

import javax.swing.*;
import java.awt.*;
import java.awt.event.ComponentAdapter;
import java.awt.event.ComponentEvent;
import java.io.*;
import java.net.Socket;
import java.util.Objects;

public class ClientGUI {
    private static Socket socket;
    private static final Gson gson = new Gson();
    private static JFrame mainFrame;
    private static GameState gameState;
    private static GameView gameView;

    public static void main(String[] args) {
        mainFrame = new JFrame("Schotten Totten 2 (client)");
        mainFrame.setSize(Constants.WINDOW_WIDTH, Constants.WINDOW_HEIGHT);
        mainFrame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        mainFrame.setVisible(true);

        while (true) {
            String hostIP = JOptionPane.showInputDialog(mainFrame, "Enter host IP: ", "Host IP", JOptionPane.QUESTION_MESSAGE);
            if (hostIP == null) {
                return;
            }
            try {
                socket = new Socket(hostIP, 12345);
                break;
            } catch (IOException e) {
                e.printStackTrace();
                JOptionPane.showMessageDialog(mainFrame, "Unable to connect to the server. Please check the IP and try again.", "Connection Error", JOptionPane.ERROR_MESSAGE);
            }
        }

        mainFrame.addComponentListener(new ComponentAdapter() {
            @Override
            public void componentResized(ComponentEvent e) {
                Constants.resize(mainFrame.getWidth(), mainFrame.getHeight());
                if (gameView != null) {
                    gameView.updateLayout(ClientGUI::onWallClicked);
                }
                updateUI();
            }
        });

        JLabel label = new JLabel("Host is choosing role...");
        label.setHorizontalAlignment(SwingConstants.CENTER);
        mainFrame.add(label, BorderLayout.CENTER);
        mainFrame.revalidate();
        mainFrame.repaint();
        listenForGameState();
    }

    private static void listenForGameState() {
        new Thread(() -> {
            try (InputStream input = socket.getInputStream();
                 BufferedReader reader = new BufferedReader(new InputStreamReader(input));
            ) {
                String json;
                while ((json = reader.readLine()) != null) {
                    System.out.println("receiving gamestate:" + System.currentTimeMillis());
                    gameState = gson.fromJson(json, GameState.class);
                    gameView = new GameView(gameState, ClientGUI::onWallClicked);
                    updateUI();

                    if (gameState.getWinner() != Winner.NONE) {
                        SwingUtilities.invokeLater(() ->
                                JOptionPane.showMessageDialog(mainFrame,
                                        gameState.getWinner() == Winner.ATTACKER ? "Attacker wins!" : "Defender wins!",
                                        "Game Over", JOptionPane.INFORMATION_MESSAGE));
                    }
                }
            } catch (IOException e) {
                e.printStackTrace();
            }
        }).start();
    }

    public static void updateUI() {
        mainFrame.getContentPane().removeAll();
        mainFrame.add(Objects.requireNonNullElseGet(gameView, () -> new JLabel("Host is choosing role...")));
        mainFrame.revalidate();
        mainFrame.repaint();
    }

    public static void onWallClicked(Wall wall) {
        Card card = gameView.getSelectedCard();
        if (card != null) {
            if (gameState.isClientTurn()) {
                ClientMove move = new ClientMove(card, wall.getWallIndex());
                gameView.unselectCard();
                try {
                    PrintWriter out = new PrintWriter(socket.getOutputStream(), true);
                    String jsonMove = gson.toJson(move);
                    System.out.println("before sending json: " + System.currentTimeMillis());
                    out.println(jsonMove);
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }
    }
}

