use std::io::{BufRead, BufReader, Write, Result, ErrorKind};
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
    pub player_streams: Vec<Option<PlayerStream>>,
    listener: TcpListener,
}

impl Network {
    /// Initializes the server and binds the TCP listener.
    pub fn new() -> Result<Self> {
        let ip = local_ip().unwrap_or_else(|e| {
            eprintln!("Couldn't get local IP: {}. Defaulting to 127.0.0.1", e);
            "127.0.0.1".parse::<std::net::IpAddr>().unwrap()
        });

        let listener = TcpListener::bind("0.0.0.0:4000")?;
        listener.set_nonblocking(true)?;

        println!("✅ Server started! Host IP: {ip}:4000");
        println!("Waiting for players to connect...");

        Ok(Self {
            player_streams: vec![None, None],
            listener,
        })
    }

    /// Checks for and accepts new connections to fill empty player slots.
    /// This should be called periodically (e.g., in your main game loop).
    pub fn accept_new_players(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    if let Some((i, slot)) = self.player_streams.iter_mut().enumerate().find(|(_, s)| s.is_none()) {
                        println!("Player {} connected!", i);
                        match PlayerStream::new(stream) {
                            Ok(player_stream) => *slot = Some(player_stream),
                            Err(e) => eprintln!("Error creating player stream: {}", e),
                        }
                    } else {
                        // This happens if a third player tries to connect.
                        println!("A third player tried to connect, but the game is full.");
                        let _ = writeln!(stream, "Server is full.");
                    }
                }
                // This error is expected in non-blocking mode; it just means no new client is waiting.
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    break;
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }

    /// Sends the current game state to all connected players.
    /// Handles disconnections gracefully if a write operation fails.
    pub fn send_game_state(&mut self, game_state: &GameState, turn: usize) {
        for i in 0..self.player_streams.len() {
            if let Some(player_stream) = &mut self.player_streams[i] {
                let client_state = game_state.create_client_game_state(i, turn == i);
                let json = match serde_json::to_string(&client_state) {
                    Ok(j) => j,
                    Err(e) => {
                        eprintln!("Error serializing game state: {}", e);
                        continue;
                    }
                };

                // If writing to the stream fails, it means the client has disconnected.
                if writeln!(player_stream.stream, "{}", json).is_err() {
                    println!("Player {} disconnected (write error).", i);
                    // Set their slot back to `None` so a new player can join.
                    self.player_streams[i] = None;
                }
            }
        }
    }

    /// Waits for a move from a specific player.
    /// Returns `None` if the player disconnects instead of sending a move.
    pub fn wait_for_move(&mut self, player_index: usize) -> Option<ClientMove> {
        let player_stream = match self.player_streams.get_mut(player_index).and_then(|s| s.as_mut()) {
            Some(ps) => ps,
            None => return None, // Player is not connected.
        };

        let mut line = String::new();
        match player_stream.reader.read_line(&mut line) {
            // `Ok(0)` indicates the client closed the connection gracefully (EOF).
            Ok(0) => {
                println!("Player {} disconnected gracefully.", player_index);
                self.player_streams[player_index] = None;
                None
            }
            Ok(_) => {
                // We received data, so we try to parse it.
                match serde_json::from_str(line.trim()) {
                    Ok(client_move) => Some(client_move),
                    Err(e) => {
                        eprintln!("Failed to parse move from player {}: {}", player_index, e);
                        // Invalid data from a client can be treated as a disconnection.
                        self.player_streams[player_index] = None;
                        None
                    }
                }
            }
            // Any other read error also indicates a disconnection.
            Err(_) => {
                println!("Player {} disconnected (read error).", player_index);
                self.player_streams[player_index] = None;
                None
            }
        }
    }

    /// A helper to check if all player slots are filled.
    pub fn all_players_connected(&self) -> bool {
        self.player_streams.iter().all(|s| s.is_some())
    }
}