One person download the host folder. If you have an IDE, you can run the source code. Otherwise, run the jar in terminal by going into the directory/folder with the jar file and typing "java -jar host.jar [port]". Replace [port] with a 5 digit number less than 65535 (ex 12345).

One person download the client folder. Same instructions as the host, but you may run the program in the terminal by typing "java -jar client.jar [host IP] [port]". Replace host IP with the host's IP address (can be found using whatismyipaddress.com), and replace [port] with the same number as the host's port.

If running in IntelliJ, make a custom run configuration with the same parameters ([port]/[IP] [port]).
