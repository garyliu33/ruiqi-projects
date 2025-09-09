package com.st.server;

import java.io.InputStream;
import java.io.OutputStream;
import java.io.IOException;

public interface NetworkSocket extends AutoCloseable {
    InputStream getInputStream() throws IOException;
    OutputStream getOutputStream() throws IOException;
    void close() throws IOException;
    boolean isClosed();
}
