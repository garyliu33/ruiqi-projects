package com.st.server;

import com.st.common.Card;
import com.st.common.ClientMove;
import com.st.common.GameState;
import com.st.common.Role;
import com.st.common.Winner;
import com.st.proto.Participant;
import com.st.proto.GameService.ClientToServer;
import com.st.proto.GameService.ServerToClient;
import com.st.proto.SchottenTotten2ServiceGrpc.SchottenTotten2ServiceImplBase;
import com.st.proto.Participant.ClientTypeProto;
import com.st.proto.ClientMove.ClientMoveProto;
import com.st.proto.Participant.ClientDeclarationProto;
import io.grpc.ServerBuilder;
import io.grpc.Status;
import io.grpc.StatusRuntimeException;
import io.grpc.stub.StreamObserver;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Set;
import java.util.UUID;
import java.util.concurrent.ConcurrentHashMap;

public class Server extends SchottenTotten2ServiceImplBase {
    private final List<PlayerConnection> players = Collections.synchronizedList(new ArrayList<>()); // Should be thread-safe
    private final Set<StreamObserver<ServerToClient>> watchers = ConcurrentHashMap.newKeySet();

    // This map will store the stream observers for connected players, keyed by their UUID.
    // This allows us to send game state updates to the correct player.
    private final ConcurrentHashMap<UUID, StreamObserver<ServerToClient>> playerObservers = new ConcurrentHashMap<>();
    private GameController gameController;

    @Override
    public StreamObserver<ClientToServer> gameStream(StreamObserver<ServerToClient> responseObserver) {
        return new StreamObserver<>() {
            private UUID clientUuid = null; // To identify which player this stream belongs to

            // Find the player associated with this stream.
            private PlayerConnection getPlayer() { return findClientByUuid(clientUuid); }

            @Override
            public void onNext(ClientToServer request) {
                switch (request.getMessageCase()) {
                    case DECLARATION:
                        handleDeclaration(request.getDeclaration(), responseObserver);
                        break;
                    case RECONNECT:
                        try {
                            UUID uuid = UUID.fromString(request.getReconnect().getUuid());
                            PlayerConnection existingPlayer = findClientByUuid(uuid);
                            if (existingPlayer != null) {
                                System.out.println("Player " + uuid + " reconnected.");
                                this.clientUuid = uuid;
                                playerObservers.put(uuid, responseObserver);
                                sendGameStateToPlayer(uuid);
                            } else {
                                System.out.println("Reconnect attempt from unknown UUID " + uuid + ". Rejecting.");
                                responseObserver.onError(new StatusRuntimeException(Status.NOT_FOUND.withDescription("Unknown UUID")));
                            }
                        } catch (IllegalArgumentException e) {
                            System.out.println("Invalid UUID format received. Rejecting client.");
                            responseObserver.onError(new StatusRuntimeException(Status.INVALID_ARGUMENT.withDescription("Invalid UUID format")));
                        }
                        break;
                    case MOVE:
                        if (gameController != null) {
                            PlayerConnection player = getPlayer();
                            if (player != null) {
                                gameController.processMove(ClientMove.fromProto(request.getMove()), player.getRole());
                                broadcastGameState();
                            }
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
                cleanupObserver(responseObserver, clientUuid);
            }

            @Override
            public void onCompleted() {
                System.out.println("Client disconnected.");
                cleanupObserver(responseObserver, clientUuid);
                responseObserver.onCompleted();
            }

            private void handleDeclaration(ClientDeclarationProto declaration, StreamObserver<ServerToClient> observer) {
                if (declaration.getClientType() == ClientTypeProto.PLAYER) {
                    PlayerConnection newPlayer = handleNewPlayer(observer);
                    if (newPlayer != null) {
                        this.clientUuid = newPlayer.getUuid();
                        Participant.ClientDeclarationResponseProto response = Participant.ClientDeclarationResponseProto.newBuilder()
                                .setStatus(Participant.ClientDeclarationResponseProto.Status.SUCCESS)
                                .setAssignedRole(newPlayer.getRole().toProto())
                                .setUuid(newPlayer.getUuid().toString())
                                .build();
                        observer.onNext(ServerToClient.newBuilder().setDeclarationResponse(response).build());
                    }
                } else { // WATCHER
                    System.out.println("Watcher connected.");
                    watchers.add(observer);
                    sendGameStateToObserver(observer); // Send initial state to the new watcher
                }
            }
        };
    }

    private PlayerConnection findClientByUuid(UUID uuid) {
        for (PlayerConnection client : players) {
            if (client.getUuid() != null && client.getUuid().equals(uuid)) {
                return client;
            }
        }
        return null;
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
        playerObservers.put(newClient.getUuid(), responseObserver);

        // If both players are now connected, start the game.
        if (players.size() == 2) {
            System.out.println("Both players connected. Starting game.");
            gameController = new GameController();
            gameController.startGame();
            broadcastGameState();
        } else {
            sendGameStateToPlayer(newClient.getUuid()); // Send initial state to the first player
        }
        return newClient;
    }

    private void cleanupObserver(StreamObserver<ServerToClient> responseObserver, UUID uuid) {
        if (uuid != null) {
            playerObservers.remove(uuid, responseObserver);
        }
        watchers.remove(responseObserver);
    }

    private PlayerConnection findPlayerByRole(Role role) {
        return players.stream().filter(p -> p.getRole() == role).findFirst().orElse(null);
    }

    private void sendGameStateToPlayer(UUID playerUuid) {
        StreamObserver<ServerToClient> observer = playerObservers.get(playerUuid);
        if (observer == null) return;

        ServerToClient update;
        if (gameController != null) {
            PlayerConnection player = findClientByUuid(playerUuid);
            if (player == null) return;
            GameState state = gameController.createGameStateForPlayer(player.getRole());
            update = ServerToClient.newBuilder().setGameState(state.toProto()).build();
        } else {
            // Game hasn't started, send an empty state.
            GameState emptyState = new GameState();
            update = ServerToClient.newBuilder().setGameState(emptyState.toProto()).build();
        }
        observer.onNext(update);
    }

    private void sendGameStateToObserver(StreamObserver<ServerToClient> observer) {
        ServerToClient update;
        if (gameController != null) {
            // Watchers get the full, unobfuscated game state.
            update = ServerToClient.newBuilder().setGameState(gameController.getFullGameState().toProto()).build();
        } else {
            // Game hasn't started, send an empty state.
            GameState emptyState = new GameState();
            update = ServerToClient.newBuilder().setGameState(emptyState.toProto()).build();
        }
        observer.onNext(update);
    }

    private void broadcastGameState() {
        if (gameController == null) return;

        // Send personalized game state to each player
        for (UUID playerUuid : playerObservers.keySet()) {
            sendGameStateToPlayer(playerUuid);
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
