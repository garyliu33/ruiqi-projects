import java.io.*;
import java.net.ServerSocket;
import java.net.Socket;
import java.util.function.Consumer;
import com.google.gson.Gson;

public class Network {
    private PrintWriter out;
    private BufferedReader in;
    private Thread listenerThread;
    private boolean listening;
    private final Gson gson = new Gson();
    private Consumer<ClientMove> moveHandler;
    private GameState state;
    private final ServerSocket serverSocket;

    public Network(ServerSocket serverSocket) throws IOException {
        Socket clientSocket = serverSocket.accept();
        out = new PrintWriter(clientSocket.getOutputStream(), true);
        in = new BufferedReader(new InputStreamReader(clientSocket.getInputStream()));
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
                String json;
                while (listening && (json = in.readLine()) != null) {
                    ClientMove move = gson.fromJson(json, ClientMove.class);
                    if (move != null && moveHandler != null) {
                        moveHandler.accept(move);
                    }
                }

                if (listening) {
                    waitForReconnect();
                }
            } catch (IOException e) {
                waitForReconnect();
            }
        });
        listenerThread.start();
    }

    public void sendGameState(GameState state) {
        this.state = state;
        String json = gson.toJson(state);
        out.println(json);
    }

    private void waitForReconnect() {
        HostGUI.clientDisconnected();
        try {
            out.close();
            in.close();
        } catch (IOException e) {
            e.printStackTrace();
        }

        try {
            Socket newSocket = serverSocket.accept();
            out = new PrintWriter(newSocket.getOutputStream(), true);
            in = new BufferedReader(new InputStreamReader(newSocket.getInputStream()));
            HostGUI.displayGameState();
            sendGameState(state);
            listenForClientMessages();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
