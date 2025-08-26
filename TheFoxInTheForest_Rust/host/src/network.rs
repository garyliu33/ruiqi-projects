use std::io::{BufRead, BufReader, Write, Result};
use std::net::{TcpListener, TcpStream};
use local_ip_address::local_ip;
use common::client_move::ClientMove;
use crate::game_state::GameState;

pub struct PlayerStream {
    pub stream: TcpStream,
    pub reader: BufReader<TcpStream>
}

impl PlayerStream {
    pub fn new(stream: TcpStream) -> Result<Self> {
        let reader = BufReader::new(stream.try_clone()?);
        Ok(Self { stream, reader })
    }
}

pub struct Network {
    pub player_streams: Vec<PlayerStream>
}

impl Network {
    pub fn new() -> Self {
        Self { player_streams: Vec::new() }
    }

    pub fn start_server(&mut self) -> Result<()> {
        let ip = local_ip().unwrap();
        let listener = TcpListener::bind("0.0.0.0:4000")?;
        println!("Host IP: {ip}");

        for stream in listener.incoming().take(2) {
            let stream = stream?;
            println!("Client connected!");
            self.player_streams.push(PlayerStream::new(stream)?);
        }

        if self.player_streams.len() == 2 {
            Ok(())
        } else {
            eprintln!("Error: Expected exactly 2 players, but got {}", self.player_streams.len());
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid number of players"))
        }
    }

    pub fn send_game_state(&mut self, game_state: &GameState, turn: usize) -> Result<()> {
        for i in 0..self.player_streams.len() {
            let json = serde_json::to_string(&game_state.create_client_game_state(i, turn == i))?;
            writeln!(self.player_streams[i].stream, "{}", json).expect("Failed to write message");
        }
        Ok(())
    }

    pub fn wait_for_move(&mut self, player_index: usize) -> ClientMove {
        let reader = &mut self.player_streams[player_index].reader;
        let mut line = String::new();
        reader.read_line(&mut line).expect("Failed to read line");
        serde_json::from_str(line.trim()).expect("Failed to change to ClientMove")
    }
}