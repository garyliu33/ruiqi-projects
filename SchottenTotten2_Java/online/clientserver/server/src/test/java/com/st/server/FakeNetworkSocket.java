package com.st.server;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;

public class FakeNetworkSocket implements NetworkSocket {

    private final ByteArrayInputStream inputStream;
    private final ByteArrayOutputStream outputStream = new ByteArrayOutputStream();
    private boolean closed = false;

    public FakeNetworkSocket(byte[] inputData) {
        this.inputStream = new ByteArrayInputStream(inputData);
    }

    @Override
    public InputStream getInputStream() throws IOException {
        return inputStream;
    }

    @Override
    public OutputStream getOutputStream() throws IOException {
        return outputStream;
    }

    @Override
    public void close() throws IOException {
        closed = true;
        inputStream.close();
        outputStream.close();
    }

    @Override
    public boolean isClosed() {
        return closed;
    }

    public byte[] getOutputData() {
        return outputStream.toByteArray();
    }
}
