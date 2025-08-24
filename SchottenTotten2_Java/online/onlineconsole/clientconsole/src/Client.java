import java.util.*;
import java.io.*;
import java.net.*;

public class Client {
    public static void main(String[] args) throws IOException {
        Socket socket = new Socket(args[0], Integer.parseInt(args[1]));
        System.out.println("connected\n");
        BufferedReader in = new BufferedReader(new InputStreamReader(socket.getInputStream()));
        PrintWriter out = new PrintWriter(socket.getOutputStream(), true);
        Scanner scan = new Scanner(System.in);

        while (true) {
            String line = in.readLine();
            if (line == null) {
                break;
            }
            line = line.replace("\\n", "\n");
            if (line.startsWith("GET_INPUT")) {
                while (System.in.available() > 0) {
                    System.in.read();
                }
                System.out.print(line.substring("GET_INPUT".length()));
                out.println(scan.nextLine());
            } else if (line.startsWith("END_PROGRAM")) {
                System.out.print(line.substring("END_PROGRAM".length()));
                break;
            } else if (line.startsWith("NO_LN")){
                System.out.print(line.substring("NO_LN".length()));
            } else {
                System.out.println(line);
            }
        }

        socket.close();
    }
}
