package com.st.client;

import java.awt.BorderLayout;
import java.awt.event.ComponentAdapter;
import java.awt.event.ComponentEvent;
import java.util.Objects;

import javax.swing.JFrame;
import javax.swing.JLabel;
import javax.swing.JOptionPane;
import javax.swing.SwingConstants;
import javax.swing.SwingUtilities;

import com.st.common.Card;
import com.st.common.ClientMove;
import com.st.common.Constants;
import com.st.common.GameState;
import com.st.common.Role;
import com.st.common.Wall;
import com.st.proto.GameService.ClientToServer;
import com.st.proto.GameService.ServerToClient;
import com.st.proto.Participant;
import com.st.proto.SchottenTotten2ServiceGrpc;

import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import io.grpc.stub.StreamObserver;

public class ClientGUI {
    private static JFrame mainFrame;
    private static GameState gameState;
    private static GameView gameView;
    private static StreamObserver<ClientToServer> toServerStream;

    public static void main(String[] args) {
        mainFrame = new JFrame("Schotten Totten 2 (client)");
        mainFrame.setSize(Constants.WINDOW_WIDTH, Constants.WINDOW_HEIGHT);
        mainFrame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        mainFrame.setVisible(true);

        ManagedChannel channel = null;
        while (true) {
            String hostIP = JOptionPane.showInputDialog(mainFrame, "Enter host IP: ", "Host IP",
                    JOptionPane.QUESTION_MESSAGE);
            if (hostIP == null) {
                return;
            }
            if (hostIP.isEmpty()) {
                hostIP = "localhost";
            }
            try {
                channel = ManagedChannelBuilder.forTarget("dns:///"+hostIP+":12345")
                        .usePlaintext()
                        .build();
                break;
            } catch (Exception e) {
                e.printStackTrace();
                JOptionPane.showMessageDialog(mainFrame,
                        "Unable to connect to the server. Please check the IP and try again.",
                        "Connection Error", JOptionPane.ERROR_MESSAGE);
            }
        }
        connectToServer(channel);

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

        JLabel label = new JLabel("Connecting to server...");
        label.setHorizontalAlignment(SwingConstants.CENTER);
        mainFrame.add(label, BorderLayout.CENTER);
        mainFrame.revalidate();
        mainFrame.repaint();
    }
    private static void connectToServer(ManagedChannel channel) {
        SchottenTotten2ServiceGrpc.SchottenTotten2ServiceStub asyncStub = SchottenTotten2ServiceGrpc.newStub(channel);
        toServerStream = asyncStub.gameStream(new StreamObserver<>() {
            @Override
            public void onNext(ServerToClient value) {
                SwingUtilities.invokeLater(() -> handleServerMessage(value));
            }

            @Override
            public void onError(Throwable t) {
                t.printStackTrace();
                JOptionPane.showMessageDialog(mainFrame, "Connection to server lost: " + t.getMessage(), "Connection Error", JOptionPane.ERROR_MESSAGE);
            }

            @Override
            public void onCompleted() {
                JOptionPane.showMessageDialog(mainFrame, "Server has closed the connection.", "Connection Closed", JOptionPane.INFORMATION_MESSAGE);
            }
        });

        // Declare ourselves as a player
        Participant.ClientDeclarationProto declaration = Participant.ClientDeclarationProto.newBuilder()
                .setClientType(Participant.ClientTypeProto.PLAYER)
                .build();
        toServerStream.onNext(ClientToServer.newBuilder().setDeclaration(declaration).build());
    }

    private static void handleServerMessage(ServerToClient message) {
        switch (message.getMessageCase()) {
            case GAME_STATE:
                gameState = GameState.fromProto(message.getGameState());
                updateUI();
                break;
            case DECLARATION_RESPONSE:
                Participant.ClientDeclarationResponseProto response = message.getDeclarationResponse();
                if (response.getStatus() == Participant.ClientDeclarationResponseProto.Status.SUCCESS) {
                    Role role = response.getAssignedRole() == Participant.RoleProto.ATTACKER_ROLE ? Role.ATTACKER : Role.DEFENDER;
                    mainFrame.setTitle("Schotten Totten 2 (Client - " + role + ")");
                } else if (response.getStatus() == Participant.ClientDeclarationResponseProto.Status.GAME_FULL) {
                    JOptionPane.showMessageDialog(mainFrame, "Game is full. Cannot join.", "Game Full", JOptionPane.WARNING_MESSAGE);
                }
                break;
        }
    }

    public static void updateUI() {
        mainFrame.getContentPane().removeAll();
        if (gameState != null) {
            gameView = new GameView(gameState, ClientGUI::onWallClicked);
        }

        mainFrame.add(Objects.requireNonNullElseGet(gameView, () -> new JLabel("Waiting for game to start...")));
        mainFrame.revalidate();
        mainFrame.repaint();
    }

    public static void onWallClicked(Wall wall) {
        Card card = gameView.getSelectedCard();
        if (card != null) {
            if (gameState.isClientTurn()) {
                ClientMove move = new ClientMove(card, wall.getWallIndex());
                gameView.unselectCard();
                ClientToServer request = ClientToServer.newBuilder().setMove(move.toProto()).build();
                toServerStream.onNext(request);
            }
        }
    }
}
