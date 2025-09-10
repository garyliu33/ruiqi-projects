use std::io::{BufRead, BufReader, Write, Result, ErrorKind};
use std::net::{TcpListener, TcpStream};
use local_ip_address::local_ip;
use common::client_move::ClientMove;
use common::server_message::{ClientGameState, ServerMessage};

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
    game_started: bool
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

        let mut player_streams = Vec::with_capacity(6);
        for _ in 0..6 {
            player_streams.push(None);
        }

        Ok(Self {
            player_streams,
            listener,
            game_started: false
        })
    }

    pub fn start_game(&mut self) {
        self.game_started = true;

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

    /// Sends the current game state to one player
    pub fn send_game_state(&mut self, state: ClientGameState, id: usize) {
        if let Some(player_stream) = &mut self.player_streams[id] {
            let json = match serde_json::to_string(&ServerMessage::GameState(state)) {
                Ok(j) => j,
                Err(e) => {
                    panic!("Error serializing game state: {}", e);
                }
            };

            // If writing to the stream fails, it means the client has disconnected.
            if writeln!(player_stream.stream, "{}", json).is_err() {
                println!("Player {} disconnected (write error).", id);
                // Set their slot back to `None` so a new player can join.
                self.player_streams[id] = None;
            }
        }
    }
    
    pub fn send_win_message(&mut self, state: ClientGameState, winner: usize, id: usize) {
        let mut msg = String::new();
        if id == winner {
            msg.push_str("You win!");
        } else {
            msg.push_str("You lose.")
        }
        
        if let Some(player_stream) = &mut self.player_streams[id] {
            let json = match serde_json::to_string(&ServerMessage::GameOver(state, msg)) {
                Ok(j) => j,
                Err(e) => {
                    panic!("Error serializing game state: {}", e);
                }
            };

            // If writing to the stream fails, it means the client has disconnected.
            if writeln!(player_stream.stream, "{}", json).is_err() {
                println!("Player {} disconnected (write error).", id);
                // Set their slot back to `None` so a new player can join.
                self.player_streams[id] = None;
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

    /// A helper to get the current number of connected players.
    pub fn get_connected_player_count(&self) -> usize {
        self.player_streams.iter().filter(|s| s.is_some()).count()
    }
}