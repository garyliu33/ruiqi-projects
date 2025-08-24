import java.io.*;

public class Input {
    private final BufferedReader reader;

    public Input(BufferedReader reader) {
        this.reader = reader;
    }

    public String readLine() throws IOException {
        String str = reader.readLine().trim();
        if (str.equalsIgnoreCase("quit")) {
            System.exit(0);
        }
        return str;
    }

    public boolean ready() throws IOException {
        return reader.ready();
    }

    public void clear() throws IOException {
        while (reader.ready()) {
            reader.readLine();
        }
    }
}
