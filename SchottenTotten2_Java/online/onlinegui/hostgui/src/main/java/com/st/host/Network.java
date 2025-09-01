package com.st.host;

import java.io.*;
import java.net.ServerSocket;
import java.net.Socket;
import java.util.function.Consumer;

import com.google.protobuf.Message;
import com.google.protobuf.MessageLite;
import com.st.proto.ClientMove.ClientMoveProto;
import com.st.proto.GameState.GameStateProto;

public class Network {
    private OutputStream out;
    private InputStream in;
    private Thread listenerThread;
    private boolean listening;
    private Consumer<ClientMove> moveHandler;
    private GameState state;
    private final ServerSocket serverSocket;

    public Network(ServerSocket serverSocket) throws IOException {
        Socket clientSocket = serverSocket.accept();
        out = clientSocket.getOutputStream();
        in = clientSocket.getInputStream();
        this.serverSocket = serverSocket;
        listenForClientMessages();
    }

    public void setMoveHandler(Consumer<ClientMove> moveHandler) {
        this.moveHandler = moveHandler;
    }

    private void listenForClientMessages() {
        listening = false;
        if (listenerThread != null && listenerThread.isAlive()) {
            try {
                listenerThread.join(1000);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }

        listening = true;
        listenerThread = new Thread(() -> {
            try {
                ClientMove move = null;
                while (listening && (move = readClientMove()) != null) {
                    if (moveHandler != null) {
                        moveHandler.accept(move);
                    }
                }

                if (listening) {
                    waitForReconnect();
                }
            } catch (IOException e) {
                e.printStackTrace();
                waitForReconnect();
            }
        });
        listenerThread.start();
    }

    public void sendGameState(GameState state) {
        this.state = state;
        try {
            state.toProto().writeDelimitedTo(out);
        } catch (IOException ex) {
            ex.printStackTrace();
        }
    }

    public ClientMove readClientMove() throws IOException {
        return ClientMove.fromProto(ClientMoveProto.parseDelimitedFrom(in));
    }

    private void waitForReconnect() {
        try {
            out.close();
            in.close();
        } catch (IOException e) {
            e.printStackTrace();
        }

        try {
            Socket newSocket = serverSocket.accept();
            out = newSocket.getOutputStream();
            in = newSocket.getInputStream();
            HostGUI.displayGameState();
            sendGameState(state);
            listenForClientMessages();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
