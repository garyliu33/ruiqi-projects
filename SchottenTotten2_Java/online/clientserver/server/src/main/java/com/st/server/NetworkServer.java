package com.st.server;

import java.io.IOException;

public interface NetworkServer extends AutoCloseable {
    NetworkSocket accept() throws IOException;
    int getLocalPort();
    void close() throws IOException;
    boolean isClosed();
}
