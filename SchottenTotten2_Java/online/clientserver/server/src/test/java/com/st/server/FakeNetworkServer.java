package com.st.server;

import java.io.IOException;
import java.util.LinkedList;
import java.util.Queue;

public class FakeNetworkServer implements NetworkServer {
    private final Queue<FakeNetworkSocket> incomingConnections = new LinkedList<>();
    private boolean closed = false;

    public void newConnection(byte[] data) {
        incomingConnections.add(new FakeNetworkSocket(data));
    }

    @Override
    public NetworkSocket accept() throws IOException {
        if (closed) {
            throw new IOException("Server is closed");
        }
        return incomingConnections.poll();
    }

    @Override
    public void close() throws IOException {
        closed = true;
    }

    @Override
    public boolean isClosed() {
        return closed;
    }

    @Override
    public int getLocalPort() {
        return 0;
    }
}
