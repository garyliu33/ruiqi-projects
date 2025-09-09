package com.st.server;

import com.st.common.Role;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.UUID;

public class Client {

    public enum ClientType {
        PLAYER,
        WATCHER
    }

    private NetworkSocket socket;
    private InputStream inputStream;
    private OutputStream outputStream;
    private final ClientType clientType;

    // Player-specific fields
    private final UUID uuid;
    private Role role;

    /**
     * Constructor for a PLAYER client.
     */
    public Client(NetworkSocket socket, Role role) throws IOException {
        this.socket = socket;
        this.inputStream = socket.getInputStream();
        this.outputStream = socket.getOutputStream();
        this.clientType = ClientType.PLAYER;
        this.uuid = UUID.randomUUID();
        this.role = role;
    }

    /**
     * Constructor for a WATCHER client.
     */
    public Client(NetworkSocket socket) throws IOException {
        this.socket = socket;
        this.inputStream = socket.getInputStream();
        this.outputStream = socket.getOutputStream();
        this.clientType = ClientType.WATCHER;
        this.uuid = null; // Watchers don't have a UUID
        this.role = null; // Watchers don't have a role
    }

    public void reconnect(NetworkSocket newSocket) throws IOException {
        if (socket != null && !socket.isClosed()) {
            socket.close();
        }
        this.socket = newSocket;
        this.inputStream = newSocket.getInputStream();
        this.outputStream = newSocket.getOutputStream();
    }

    public NetworkSocket getSocket() {
        return socket;
    }

    public InputStream getInputStream() {
        return inputStream;
    }

    public OutputStream getOutputStream() {
        return outputStream;
    }

    public ClientType getClientType() {
        return clientType;
    }

    public UUID getUuid() {
        return uuid;
    }

    public Role getRole() {
        return role;
    }
}
