use std::io::Write;
use std::io::Result;
use std::net::TcpListener;
use common::Message;
use local_ip_address::local_ip;
use crate::game::Game;
use crate::player::Player;

pub fn start_server() -> Result<()> {
    let ip = local_ip().unwrap();
    let listener = TcpListener::bind("0.0.0.0:4000")?;
    println!("Host IP: {ip}");

    let mut streams = Vec::new();

    for stream in listener.incoming().take(2) {
        let stream = stream?;
        println!("Client connected!");
        streams.push(stream);
    }

    if streams.len() == 2 {
        let mut game = Game::new(streams.remove(0), streams.remove(0));
        game.start();
    } else {
        eprintln!("Error: Expected exactly 2 players, but got {}", streams.len());
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid number of players"));
    }

    Ok(())
}

pub fn send_message(player: &mut Player, msg: &Message) -> Result<()> {
    let json = serde_json::to_string(msg)?;
    writeln!(player.stream, "{}", json).expect("Failed to write message");
    Ok(())
}