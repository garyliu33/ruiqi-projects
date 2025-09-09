package com.st.server;

import java.io.IOException;
import java.net.ServerSocket;

public class JavaNetNetworkServer implements NetworkServer {
    private final ServerSocket serverSocket;

    public JavaNetNetworkServer(int port) throws IOException {
        this.serverSocket = new ServerSocket(port);
    }

    @Override
    public NetworkSocket accept() throws IOException {
        return new JavaNetNetworkSocket(serverSocket.accept());
    }

    @Override
    public int getLocalPort() {
        return serverSocket.getLocalPort();
    }

    @Override
    public void close() throws IOException {
        serverSocket.close();
    }

    @Override
    public boolean isClosed() {
        return serverSocket.isClosed();
    }
}
