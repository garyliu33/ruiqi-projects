package com.st.server;

import com.st.common.Role;
import java.util.UUID;

public class PlayerConnection {
    private final Role role;

    public PlayerConnection(Role role) {
        this.role = role;
    }

    public Role getRole() {
        return role;
    }
}