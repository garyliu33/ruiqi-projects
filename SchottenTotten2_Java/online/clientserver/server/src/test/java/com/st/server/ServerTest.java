package com.st.server;

import com.st.common.Role;
import com.st.proto.GameService.ClientToServer;
import com.st.proto.GameService.ServerToClient;
import com.st.proto.Participant;
import com.st.proto.Participant.ClientDeclarationProto;
import io.grpc.stub.StreamObserver;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.concurrent.ConcurrentHashMap;

import static org.junit.jupiter.api.Assertions.*;

class ServerTest {

    private Server server;
    private FakeStreamObserver fakeServerObserver; // This observer represents the server's view of the client stream
    private StreamObserver<ClientToServer> clientStream; // This observer is what the test uses to send messages to the server

    @BeforeEach
    void setUp() {
        server = new Server();
        fakeServerObserver = new FakeStreamObserver();
        clientStream = server.gameStream(fakeServerObserver);
    }

    @Test
    void testWatcherConnection() {
        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder().setClientType(com.st.proto.Participant.ClientTypeProto.WATCHER).build();
        ClientToServer request = ClientToServer.newBuilder().setDeclaration(declaration).build();
        clientStream.onNext(request);

        // Watchers are not added to the players list, but to the watchers set.
        assertEquals(0, getPlayers(server).size());
        assertEquals(1, getWatchers(server).size());

        // A watcher connecting before the game starts should receive an empty game state.
        List<ServerToClient> messages = fakeServerObserver.getReceivedMessages();
        assertEquals(1, messages.size(), "Watcher should receive exactly one message.");
        assertTrue(messages.get(0).hasGameState(), "Message should be a game state update.");
        // A new GameState initializes with a full deck.
        assertEquals(60, messages.get(0).getGameState().getDeckSize());

        assertNull(fakeServerObserver.getReceivedError());
    }

    @Test
    void testFirstPlayerConnectsAsAttacker() {
        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder().setClientType(com.st.proto.Participant.ClientTypeProto.PLAYER).build();
        ClientToServer request = ClientToServer.newBuilder().setDeclaration(declaration).build();
        clientStream.onNext(request);

        List<PlayerConnection> players = getPlayers(server);
        assertEquals(1, players.size());
        assertEquals(Role.ATTACKER, players.get(0).getRole());
        assertNull(fakeServerObserver.getReceivedError());

        // Verify the success response
        List<ServerToClient> messages = fakeServerObserver.getReceivedMessages();
        assertEquals(2, messages.size(), "Should receive declaration response and initial game state.");

        // 1. Check declaration response
        assertTrue(messages.get(0).hasDeclarationResponse(), "First message should be a declaration response.");
        Participant.ClientDeclarationResponseProto response = messages.get(0).getDeclarationResponse();
        assertEquals(Participant.ClientDeclarationResponseProto.Status.SUCCESS, response.getStatus());
        assertEquals(Participant.RoleProto.ATTACKER_ROLE, response.getAssignedRole());

        // 2. Check initial game state (should be empty)
        assertTrue(messages.get(1).hasGameState(), "Second message should be a game state update.");
        assertEquals(60, messages.get(1).getGameState().getDeckSize());
    }

    @Test
    void testSecondPlayerConnectsAsDefender() {
        // First player
        PlayerConnection attacker = new PlayerConnection(Role.ATTACKER); // Manually add first player
        getPlayers(server).add(attacker);

        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder().setClientType(com.st.proto.Participant.ClientTypeProto.PLAYER).build();
        ClientToServer request = ClientToServer.newBuilder().setDeclaration(declaration).build();
        clientStream.onNext(request);

        List<PlayerConnection> players = getPlayers(server);
        assertEquals(2, players.size());
        assertEquals(Role.DEFENDER, players.get(1).getRole());
        assertNull(fakeServerObserver.getReceivedError());
    }

    @Test
    void testThirdPlayerConnection_Rejected() {
        // First player
        PlayerConnection attacker = new PlayerConnection(Role.ATTACKER); // Manually add first player
        getPlayers(server).add(attacker);
        // Second player
        PlayerConnection defender = new PlayerConnection(Role.DEFENDER); // Manually add second player
        getPlayers(server).add(defender);

        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder().setClientType(com.st.proto.Participant.ClientTypeProto.PLAYER).build();
        ClientToServer request = ClientToServer.newBuilder().setDeclaration(declaration).build();
        clientStream.onNext(request);

        List<PlayerConnection> players = getPlayers(server);
        assertEquals(2, players.size()); // No new client added

        // Instead of an error, we now expect a specific response message
        assertNull(fakeServerObserver.getReceivedError(), "Should not throw an error for a full game.");
        List<ServerToClient> messages = fakeServerObserver.getReceivedMessages();
        assertEquals(1, messages.size());
        Participant.ClientDeclarationResponseProto response = messages.get(0).getDeclarationResponse();
        assertEquals(Participant.ClientDeclarationResponseProto.Status.GAME_FULL, response.getStatus());
    }

    @SuppressWarnings("unchecked")
    private List<PlayerConnection> getPlayers(Server server) {
        try {
            java.lang.reflect.Field field = Server.class.getDeclaredField("players");
            field.setAccessible(true);
            return (List<PlayerConnection>) field.get(server);
        } catch (NoSuchFieldException | IllegalAccessException e) {
            fail("Failed to get players field from Server", e);
            return null;
        }
    }

    @SuppressWarnings("unchecked")
    private ConcurrentHashMap<Role, StreamObserver<ServerToClient>> getPlayerObservers(Server server) {
        try {
            java.lang.reflect.Field field = Server.class.getDeclaredField("playerObservers");
            field.setAccessible(true);
            return (ConcurrentHashMap<Role, StreamObserver<ServerToClient>>) field.get(server);
        } catch (NoSuchFieldException | IllegalAccessException e) {
            fail("Failed to get playerObservers field from Server", e);
            return null;
        }
    }

    @SuppressWarnings("unchecked")
    private java.util.Set<StreamObserver<ServerToClient>> getWatchers(Server server) {
        try {
            java.lang.reflect.Field field = Server.class.getDeclaredField("watchers");
            field.setAccessible(true);
            return (java.util.Set<StreamObserver<ServerToClient>>) field.get(server);
        } catch (NoSuchFieldException | IllegalAccessException e) {
            fail("Failed to get watchers field from Server", e);
            return null;
        }
    }
}