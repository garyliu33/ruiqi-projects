package com.st.server;

import com.st.common.Role;
import com.st.proto.Participant.ClientDeclarationProto;
import com.st.proto.Participant.ClientTypeProto;
import com.st.proto.Participant.RoleProto;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.io.ByteArrayOutputStream;
import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class ServerTest {

    private Server server;
    private FakeNetworkServer networkServer;

    @BeforeEach
    void setUp() {
        networkServer = new FakeNetworkServer();
        server = new Server(networkServer) {
            @Override
            public void startAccepting() {
                // Process only one client for testing
                try {
                    NetworkSocket clientSocket = networkServer.accept();
                    if (clientSocket != null) {
                        handleClient(clientSocket);
                    }
                } catch (IOException e) {
                    fail("Test failed due to IOException", e);
                }
            }
        };
    }

    private void simulateClientConnection(ClientDeclarationProto declaration) throws IOException {
        ByteArrayOutputStream outputStream = new ByteArrayOutputStream();
        declaration.writeDelimitedTo(outputStream);
        networkServer.newConnection(outputStream.toByteArray());
        server.startAccepting();
    }

    @Test
    void testWatcherConnection() throws IOException {
        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder()
                .setClientType(ClientTypeProto.WATCHER)
                .build();
        simulateClientConnection(declaration);

        List<Client> clients = getClients(server);
        assertEquals(1, clients.size());
        assertEquals(Client.ClientType.WATCHER, clients.get(0).getClientType());
        assertNull(clients.get(0).getRole());
    }

    @Test
    void testFirstPlayerConnectsAsAttacker() throws IOException {
        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder()
                .setClientType(ClientTypeProto.PLAYER)
                .setRole(RoleProto.ROLE_UNSPECIFIED)
                .build();
        simulateClientConnection(declaration);

        List<Client> clients = getClients(server);
        assertEquals(1, clients.size());
        assertEquals(Client.ClientType.PLAYER, clients.get(0).getClientType());
        assertEquals(Role.ATTACKER, clients.get(0).getRole());
    }

    @Test
    void testSecondPlayerConnectsAsDefender() throws IOException {
        // First player
        Client attacker = new Client(new FakeNetworkSocket(new byte[0]), Role.ATTACKER);
        getClients(server).add(attacker);

        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder()
                .setClientType(ClientTypeProto.PLAYER)
                .setRole(RoleProto.ROLE_UNSPECIFIED)
                .build();
        simulateClientConnection(declaration);

        List<Client> clients = getClients(server);
        assertEquals(2, clients.size());
        assertEquals(Role.DEFENDER, clients.get(1).getRole());
    }

    @Test
    void testPlayerRequestsAttackerRole_Available() throws IOException {
        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder()
                .setClientType(ClientTypeProto.PLAYER)
                .setRole(RoleProto.ATTACKER_ROLE)
                .build();
        simulateClientConnection(declaration);

        List<Client> clients = getClients(server);
        assertEquals(1, clients.size());
        assertEquals(Role.ATTACKER, clients.get(0).getRole());
    }

    @Test
    void testPlayerRequestsDefenderRole_Available() throws IOException {
        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder()
                .setClientType(ClientTypeProto.PLAYER)
                .setRole(RoleProto.DEFENDER_ROLE)
                .build();
        simulateClientConnection(declaration);

        List<Client> clients = getClients(server);
        assertEquals(1, clients.size());
        assertEquals(Role.DEFENDER, clients.get(0).getRole());
    }

    @Test
    void testPlayerRequestsAttackerRole_Taken_AssignsDefender() throws IOException {
        // First player
        Client attacker = new Client(new FakeNetworkSocket(new byte[0]), Role.ATTACKER);
        getClients(server).add(attacker);

        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder()
                .setClientType(ClientTypeProto.PLAYER)
                .setRole(RoleProto.ATTACKER_ROLE)
                .build();
        simulateClientConnection(declaration);

        List<Client> clients = getClients(server);
        assertEquals(2, clients.size());
        assertEquals(Role.DEFENDER, clients.get(1).getRole());
    }

    @Test
    void testPlayerRequestsDefenderRole_Taken_AssignsAttacker() throws IOException {
        // First player
        Client defender = new Client(new FakeNetworkSocket(new byte[0]), Role.DEFENDER);
        getClients(server).add(defender);

        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder()
                .setClientType(ClientTypeProto.PLAYER)
                .setRole(RoleProto.DEFENDER_ROLE)
                .build();
        simulateClientConnection(declaration);

        List<Client> clients = getClients(server);
        assertEquals(2, clients.size());
        assertEquals(Role.ATTACKER, clients.get(1).getRole());
    }

    @Test
    void testThirdPlayerConnection_Rejected() throws IOException {
        // First player
        Client attacker = new Client(new FakeNetworkSocket(new byte[0]), Role.ATTACKER);
        getClients(server).add(attacker);
        // Second player
        Client defender = new Client(new FakeNetworkSocket(new byte[0]), Role.DEFENDER);
        getClients(server).add(defender);

        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder()
                .setClientType(ClientTypeProto.PLAYER)
                .setRole(RoleProto.ROLE_UNSPECIFIED)
                .build();

        simulateClientConnection(declaration);

        List<Client> clients = getClients(server);
        assertEquals(2, clients.size()); // No new client added
    }

    @Test
    void testPlayerReconnect_Success() throws IOException {
        // 1. Create an initial client and add it to the server
        FakeNetworkSocket originalSocket = new FakeNetworkSocket(new byte[0]);
        Client existingClient = new Client(originalSocket, Role.ATTACKER);
        getClients(server).add(existingClient);

        // 2. Simulate a reconnection attempt with the same UUID
        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder()
                .setClientType(ClientTypeProto.PLAYER)
                .setUuid(existingClient.getUuid().toString())
                .build();

        // The new socket that the client is reconnecting with
        ByteArrayOutputStream outputStream = new ByteArrayOutputStream();
        declaration.writeDelimitedTo(outputStream);
        FakeNetworkSocket newSocket = new FakeNetworkSocket(outputStream.toByteArray());
        networkServer.newConnection(newSocket.getInputStream().readAllBytes());

        server.startAccepting();

        // 3. Assertions
        List<Client> clients = getClients(server);
        assertEquals(1, clients.size(), "A new client should not have been added.");
        assertNotSame(originalSocket, clients.get(0).getSocket(), "The client's socket should be updated to the new one.");
        assertTrue(originalSocket.isClosed(), "The original socket should be closed after reconnection.");

        newSocket.close();
    }

    @Test
    void testPlayerReconnect_UnknownUuid_Rejected() throws IOException {
        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder()
                .setClientType(ClientTypeProto.PLAYER)
                .setUuid("123e4567-e89b-12d3-a456-426614174000") // A random, unknown UUID
                .build();
        simulateClientConnection(declaration);

        List<Client> clients = getClients(server);
        assertTrue(clients.isEmpty(), "Connection with unknown UUID should be rejected.");
    }

    @Test
    void testPlayerReconnect_InvalidUuid_Rejected() throws IOException {
        ClientDeclarationProto declaration = ClientDeclarationProto.newBuilder()
                .setClientType(ClientTypeProto.PLAYER)
                .setUuid("not-a-valid-uuid")
                .build();
        simulateClientConnection(declaration);
        assertTrue(getClients(server).isEmpty(), "Connection with invalid UUID should be rejected.");
    }

    @SuppressWarnings("unchecked")
    private List<Client> getClients(Server server) {
        try {
            java.lang.reflect.Field field = Server.class.getDeclaredField("clients");
            field.setAccessible(true);
            return (List<Client>) field.get(server);
        } catch (NoSuchFieldException | IllegalAccessException e) {
            fail("Failed to get clients field from Server", e);
            return null;
        }
    }
}