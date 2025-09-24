package com.st.common;

import com.st.proto.Participant.RoleProto;

public enum Role {
    ATTACKER, DEFENDER;

    public RoleProto toProto() {
        switch (this) {
            case ATTACKER:
                return RoleProto.ATTACKER_ROLE;
            case DEFENDER:
                return RoleProto.DEFENDER_ROLE;
        }
        throw new IllegalStateException("Unknown role: " + this);
    }

    public static Role fromProto(RoleProto roleProto) {
        switch (roleProto) {
            case ATTACKER_ROLE:
                return ATTACKER;
            case DEFENDER_ROLE:
                return DEFENDER;
        }
        throw new IllegalStateException("Unknown role proto: " + roleProto);
    }
}
