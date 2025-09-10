package com.st.server;

import com.st.proto.GameService.ServerToClient;
import io.grpc.stub.StreamObserver;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.CountDownLatch;

public class FakeStreamObserver implements StreamObserver<ServerToClient> {
    private final List<ServerToClient> receivedMessages = new ArrayList<>();
    private Throwable receivedError = null;
    private boolean completed = false;
    private final CountDownLatch latch = new CountDownLatch(1);

    @Override
    public void onNext(ServerToClient value) {
        receivedMessages.add(value);
    }

    @Override
    public void onError(Throwable t) {
        receivedError = t;
        latch.countDown();
    }

    @Override
    public void onCompleted() {
        completed = true;
        latch.countDown();
    }

    public List<ServerToClient> getReceivedMessages() {
        return receivedMessages;
    }

    public Throwable getReceivedError() {
        return receivedError;
    }
}