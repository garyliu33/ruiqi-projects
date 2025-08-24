import java.util.*;
import java.io.*;
import java.net.*;

public class Display {
    private static PrintWriter clientOut;

    public static void setClientOut(PrintWriter out) {
        clientOut = out;
    }

    public static void toHost(String msg) {
        System.out.print(msg);
    }

    public static void toHostln(String msg) {
        System.out.println(msg);
    }

    public static void toClient(String msg) {
        toClient(msg, "");
    }

    public static void toClient(String msg, String prefix) {
        if (!prefix.isEmpty()) {
            clientOut.println(prefix + msg.replace("\n", "\\n"));
        } else {
            clientOut.println(msg.replace("\n", "\\n"));
        }
    }

    public static void toBoth(String msg) {
        toHost(msg);
        toClient(msg);
    }

    public static void toBoth(String msg, String prefix) {
        toHost(msg);
        toClient(msg, prefix);
    }

    public static void toBothln(String msg) {
        toHostln(msg);
        toClient(msg);
    }

    public static void toBothln(String msg, String prefix) {
        toHostln(msg);
        toClient(msg, prefix);
    }
}
