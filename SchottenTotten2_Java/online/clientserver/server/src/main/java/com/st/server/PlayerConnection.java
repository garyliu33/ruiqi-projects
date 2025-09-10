package com.st.server;

import com.st.common.Role;
import java.util.UUID;

/**
 * Represents a player's connection on the server side.
 * This class no longer manages network connections, as gRPC handles that.
 * It's now a state holder for player-specific connection data like UUID and Role.
 */
public class PlayerConnection {
    private final UUID uuid;
    private final Role role;

    public PlayerConnection(Role role) {
        this.uuid = UUID.randomUUID();
        this.role = role;
    }

    public UUID getUuid() {
        return uuid;
    }
    public Role getRole() {
        return role;
    }
}