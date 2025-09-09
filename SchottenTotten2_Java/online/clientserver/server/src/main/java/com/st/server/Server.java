package com.st.server;

import com.st.common.Role;
import com.st.proto.Participant.ClientDeclarationProto;
import com.st.proto.Participant.ClientTypeProto;
import com.st.proto.Participant.RoleProto;
import java.io.IOException;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.List;
import java.util.UUID;

public class Server {
    private final NetworkServer networkServer;
    private final List<Client> clients = new ArrayList<>();

    public Server(NetworkServer networkServer) {
        this.networkServer = networkServer;
        System.out.println("Server started on port " + networkServer.getLocalPort());
    }

    public void startAccepting() {
        new Thread(() -> {
            while (!networkServer.isClosed()) {
                try {
                    NetworkSocket clientSocket = networkServer.accept();
                    handleClient(clientSocket);
                } catch (IOException e) {
                    if (networkServer.isClosed()) {
                        System.out.println("Server socket closed, stopping accepting clients.");
                    } else {
                        e.printStackTrace();
                    }
                }
            }
        }).start();
    }

    void handleClient(NetworkSocket clientSocket) throws IOException {
        System.out.println("Client connected");

        InputStream in = clientSocket.getInputStream();
        ClientDeclarationProto declaration = ClientDeclarationProto.parseDelimitedFrom(in);

        if (declaration.getClientType() == ClientTypeProto.WATCHER) {
            Client newClient = new Client(clientSocket);
            System.out.println("Watcher connected");
            clients.add(newClient);
        } else if (declaration.getClientType() == ClientTypeProto.PLAYER) {
            handlePlayer(clientSocket, declaration);
        }
    }

    private Client findClientByUuid(UUID uuid) {
        for (Client client : clients) {
            if (client.getUuid() != null && client.getUuid().equals(uuid)) {
                return client;
            }
        }
        return null;
    }

    void handlePlayer(NetworkSocket clientSocket, ClientDeclarationProto declaration) throws IOException {
        if (declaration.hasUuid()) {
            try {
                UUID uuid = UUID.fromString(declaration.getUuid());
                Client existingClient = findClientByUuid(uuid);
                if (existingClient != null) {
                    System.out.println("Player " + uuid + " reconnected.");
                    existingClient.reconnect(clientSocket);
                    return;
                } else {
                    System.out.println("Reconnect attempt from unknown UUID " + uuid + ". Rejecting.");
                    clientSocket.close();
                    return;
                }
            } catch (IllegalArgumentException e) {
                System.out.println("Invalid UUID format received. Rejecting client.");
                clientSocket.close();
                return;
            }
        }

        boolean attackerTaken = false;
        boolean defenderTaken = false;
        for (Client c : clients) {
            if (c.getClientType() == Client.ClientType.PLAYER && c.getRole() != null) {
                if (c.getRole() == Role.ATTACKER) {
                    attackerTaken = true;
                } else if (c.getRole() == Role.DEFENDER) {
                    defenderTaken = true;
                }
            }
        }

        if (attackerTaken && defenderTaken) {
            System.out.println("Both player roles are taken. Rejecting client.");
            clientSocket.close();
            return;
        }

        Role assignedRole = null;
        RoleProto requestedRoleProto = declaration.getRole();

        if (requestedRoleProto == RoleProto.ROLE_UNSPECIFIED) {
            if (!attackerTaken) {
                assignedRole = Role.ATTACKER;
            } else {
                assignedRole = Role.DEFENDER;
            }
        } else {
            Role requestedRole = (requestedRoleProto == RoleProto.ATTACKER_ROLE)
                    ? Role.ATTACKER
                    : Role.DEFENDER;

            if (requestedRole == Role.ATTACKER) {
                if (!attackerTaken) {
                    assignedRole = Role.ATTACKER;
                } else { // Attacker taken, defender must be free
                    assignedRole = Role.DEFENDER;
                    System.out.println("Attacker role taken, assigning Defender role.");
                }
            } else { // requestedRole is DEFENDER
                if (!defenderTaken) {
                    assignedRole = Role.DEFENDER;
                } else { // Defender taken, attacker must be free
                    assignedRole = Role.ATTACKER;
                    System.out.println("Defender role taken, assigning Attacker role.");
                }
            }
        }

        Client newClient = new Client(clientSocket, assignedRole);
        System.out.println("Player connected as " + assignedRole);
        clients.add(newClient);
    }


    // You can add methods here to get streams for a specific client
    // e.g., public InputStream getInputStream(int clientIndex) throws IOException

    public static void main(String[] args) {
        try {
            Server server = new Server(new JavaNetNetworkServer(12345));
            server.startAccepting();
        } catch (IOException e) {
            System.err.println("Could not start server: " + e.getMessage());
        }
    }
}
