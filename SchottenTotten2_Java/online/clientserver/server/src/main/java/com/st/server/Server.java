package com.st.server;

import com.st.common.ClientMove;
import com.st.common.GameState;
import com.st.common.Role;
import com.st.proto.Participant;
import com.st.proto.GameService.ClientToServer;
import com.st.proto.GameService.ServerToClient;
import com.st.proto.SchottenTotten2ServiceGrpc.SchottenTotten2ServiceImplBase;
import com.st.proto.Participant.ClientTypeProto;
import com.st.proto.Participant.ClientDeclarationProto;
import io.grpc.ServerBuilder;
import io.grpc.stub.StreamObserver;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;

public class Server extends SchottenTotten2ServiceImplBase {
    private final List<PlayerConnection> players = Collections.synchronizedList(new ArrayList<>()); // Should be thread-safe
    private final Set<StreamObserver<ServerToClient>> watchers = ConcurrentHashMap.newKeySet();

    // This map will store the stream observers for connected players, keyed by their Role.
    // This allows us to send game state updates to the correct player.
    private final ConcurrentHashMap<Role, StreamObserver<ServerToClient>> playerObservers = new ConcurrentHashMap<>();
    private final GameController gameController;

    public Server() {
        gameController = new GameController();
        gameController.startGame();
    }

    @Override
    public StreamObserver<ClientToServer> gameStream(StreamObserver<ServerToClient> responseObserver) {
        return new StreamObserver<>() {
            private Role clientRole = null; // To identify which player this stream belongs to

            @Override
            public void onNext(ClientToServer request) {
                switch (request.getMessageCase()) {
                    case DECLARATION:
                        handleDeclaration(request.getDeclaration(), responseObserver);
                        break;
                    case MOVE:
                        if (gameController != null) {
                            Role moveRole = Role.fromProto(request.getMove().getRole());
                            assert clientRole == moveRole : "Client role mismatch! Stream role: " + clientRole + ", Move role: " + moveRole;
                            gameController.processMove(ClientMove.fromProto(request.getMove()), moveRole);
                            broadcastGameState();
                        }
                        break;
                    case MESSAGE_NOT_SET:
                        System.out.println("Received an empty message from a client.");
                        break;
                }
            }

            @Override
            public void onError(Throwable t) {
                System.out.println("Client stream error: " + t.getMessage());
                t.printStackTrace();
                cleanupObserver(responseObserver, clientRole);
            }

            @Override
            public void onCompleted() {
                System.out.println("Client disconnected.");
                cleanupObserver(responseObserver, clientRole);
                responseObserver.onCompleted();
            }

            private void handleDeclaration(ClientDeclarationProto declaration, StreamObserver<ServerToClient> observer) {
                if (declaration.getClientType() == ClientTypeProto.PLAYER) {
                    PlayerConnection newPlayer = handleNewPlayer(observer);
                    if (newPlayer != null) {
                        this.clientRole = newPlayer.getRole();
                        Participant.ClientDeclarationResponseProto response = Participant.ClientDeclarationResponseProto.newBuilder()
                                .setStatus(Participant.ClientDeclarationResponseProto.Status.SUCCESS)
                                .setAssignedRole(newPlayer.getRole().toProto())
                                .build();
                        observer.onNext(ServerToClient.newBuilder().setDeclarationResponse(response).build());
                        sendGameStateToPlayer(newPlayer.getRole()); // Send initial (empty) state
                    }
                } else { // WATCHER
                    System.out.println("Watcher connected.");
                    watchers.add(observer);
                    sendGameStateToObserver(observer); // Send initial state to the new watcher
                }
            }
        };
    }

    private PlayerConnection handleNewPlayer(StreamObserver<ServerToClient> responseObserver) {
        boolean attackerTaken = false;
        boolean defenderTaken = false;
        for (PlayerConnection c : players) {
            if (c.getRole() == Role.ATTACKER) {
                attackerTaken = true;
            } else if (c.getRole() == Role.DEFENDER) {
                defenderTaken = true;
            }
        }

        if (attackerTaken && defenderTaken) {
            System.out.println("Both player roles are taken. Rejecting client.");
            Participant.ClientDeclarationResponseProto response = Participant.ClientDeclarationResponseProto.newBuilder()
                    .setStatus(Participant.ClientDeclarationResponseProto.Status.GAME_FULL)
                    .build();
            responseObserver.onNext(ServerToClient.newBuilder().setDeclarationResponse(response).build());
            responseObserver.onCompleted(); // Close the stream for this client
            return null;
        }

        // Role assignment logic remains similar, but simplified as we don't need to check requested role.
        Role assignedRole = !attackerTaken ? Role.ATTACKER : Role.DEFENDER;

        PlayerConnection newClient = new PlayerConnection(assignedRole);
        System.out.println("Player connected as " + assignedRole);
        players.add(newClient);
        playerObservers.put(newClient.getRole(), responseObserver);
        return newClient;
    }

    private void cleanupObserver(StreamObserver<ServerToClient> responseObserver, Role role) {
        if (role != null) {
            playerObservers.remove(role, responseObserver);
            players.removeIf(p -> p.getRole() == role);
            System.out.println("Player " + role + " disconnected and spot is now open.");
        }
        watchers.remove(responseObserver);
    }

    private void sendGameStateToPlayer(Role playerRole) {
        StreamObserver<ServerToClient> observer = playerObservers.get(playerRole);
        assert observer != null : "Observer is null";

        GameState state = gameController.createGameStateForPlayer(playerRole);
        // System.out.println("Send game state: " + state + " to player: " + playerRole);
        ServerToClient update = ServerToClient.newBuilder().setGameState(state.toProto()).build();
        observer.onNext(update);
    }

    private void sendGameStateToObserver(StreamObserver<ServerToClient> observer) {
        ServerToClient update;
        // Watchers get the full, unobfuscated game state.
        update = ServerToClient.newBuilder().setGameState(gameController.getFullGameState().toProto()).build();
        observer.onNext(update);
    }

    private void broadcastGameState() {
        // Send personalized game state to each player
        for (Role playerRole : playerObservers.keySet()) {
            sendGameStateToPlayer(playerRole);
        }

        // Send a generic game state to all watchers
        for (StreamObserver<ServerToClient> watcherObserver : watchers) {
            sendGameStateToObserver(watcherObserver);
        }
    }

    public static void main(String[] args) throws IOException, InterruptedException {
        final Server server = new Server();
        io.grpc.Server grpcServer = ServerBuilder.forPort(12345)
                .addService(server)
                .build()
                .start();
        System.out.println("Server started, listening on " + grpcServer.getPort());
        grpcServer.awaitTermination();
    }
}
