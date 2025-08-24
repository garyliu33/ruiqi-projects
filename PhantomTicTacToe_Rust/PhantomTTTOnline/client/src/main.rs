use std::io;
use std::io::{BufRead, Write};
use std::net::TcpStream;
use common::Message;

fn main() {
    print!("Enter Host IP: ");
    io::stdout().flush().unwrap();

    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();

    let mut stream = TcpStream::connect(format!("{}:4000", ip.trim()))
        .expect("Failed to connect to host");
    println!("Welcome to Phantom Tic Tac Toe!");

    let mut reader = io::BufReader::new(stream.try_clone().unwrap());

    loop {
        let mut buffer = String::new();
        let bytes = reader.read_line(&mut buffer).expect("Failed to read line");

        if bytes == 0 {
            println!("Server disconnected.");
            break;
        }

        let msg: Message = serde_json::from_str(&buffer).unwrap();
        match process_message(&msg) {
            Some(response) => {
                let json = serde_json::to_string(&response).unwrap();
                writeln!(stream, "{}", json).expect("Failed to send message");
            }
            None => ()
        }
    }
}

fn process_message(msg: &Message) -> Option<Message> {
    println!("{}", msg.content);
    if msg.get_input {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        return Some(Message {
            content: input.trim().to_string(),
            get_input: false,
        });
    }
    None
}