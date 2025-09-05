package com.st.common;

import java.util.ArrayList;
import java.util.List;

public class PlayResult {
    private final Type type;
    private final List<Card> toDiscard;

    public enum Type {
        SUCCESS, FAILURE, ACTION
    }

    public PlayResult(Type type) {
        this(type, null);
    }

    public PlayResult(Type type, List<Card> toDiscard) {
        this.type = type;
        this.toDiscard = toDiscard == null ? new ArrayList<>() : toDiscard;
    }

    public Type getResultType() {
        return type;
    }

    public List<Card> getToDiscard() {
        return toDiscard;
    }
}
