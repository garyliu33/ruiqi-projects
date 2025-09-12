use std::io::{BufRead, BufReader, Write, Result, ErrorKind};
use std::net::{TcpListener, TcpStream};
use local_ip_address::local_ip;
use common::client_message::ClientMessage;
use common::server_message::{ClientGameState, Info, ServerMessage};

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
    pub game_started: bool
}

impl Network {
    /// Initializes the server and binds the TCP listener.
    pub fn new() -> Result<Self> {
        let ip = local_ip().unwrap_or_else(|e| {
            eprintln!("Couldn't get local IP: {}. Defaulting to 127.0.0.1", e);
            "127.0.0.1".parse::<std::net::IpAddr>().unwrap()
        });

        let listener = TcpListener::bind("0.0.0.0:12345")?;
        listener.set_nonblocking(true)?;

        println!("✅ Server started! Host IP: {ip}:12345");
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

    fn receive_message(&mut self, player_index: usize) -> (Option<ClientMessage>, bool) {
        let player_stream = match self.player_streams.get_mut(player_index).and_then(|s| s.as_mut()) {
            Some(ps) => ps,
            None => return (None, false) // Player isn't connected
        };

        let mut line = String::new();
        match player_stream.reader.read_line(&mut line) {
            Ok(0) => { // Disconnected
                println!("Player {} disconnected.", player_index);
                self.player_streams[player_index] = None;
                (None, true)
            }
            Ok(_) => { // Received data
                (serde_json::from_str::<ClientMessage>(line.trim()).ok(), false)
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                (None, false) // Normal case for non-blocking: no data available
            }
            Err(_) => { // Any other error
                println!("Player {} disconnected (read error).", player_index);
                self.player_streams[player_index] = None;
                (None, true)
            }
        }
    }

    pub fn maintain_lobby(&mut self) -> bool {
        let initial_player_count = self.get_connected_player_count();
        let mut game_started = false;

        self.accept_new_players();

        let mut disconnected_player = false;
        for i in 0..self.player_streams.len() {
            if self.player_streams[i].is_some() {
                let (message, did_disconnect) = self.receive_message(i);
                if did_disconnect {
                    disconnected_player = true;
                }

                if let Some(ClientMessage::StartGame) = message {
                    if i == 0 {
                        game_started = true;
                    }
                }
            }
        }

        // 3. If the number of players changed, notify everyone
        let current_player_count = self.get_connected_player_count();
        if current_player_count != initial_player_count || disconnected_player {
            println!("Lobby changed. Current players: {}", current_player_count);
            for id in 0..self.player_streams.len() {
                if self.player_streams[id].is_some() {
                    self.send_info(id, false);
                }
            }
        }

        game_started
    }

    /// Checks for and accepts new connections to fill empty player slots.
    /// This should be called periodically (e.g., in your main game loop).
    pub fn accept_new_players(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    if let Err(e) = stream.set_nonblocking(true) {
                        eprintln!("Failed to set stream to non-blocking: {}", e);
                        continue; // Skip this client if we can't set the mode.
                    }

                    if let Some((i, slot)) = self.player_streams.iter_mut().enumerate().find(|(_, s)| s.is_none()) {
                        println!("Player {} connected!", i);
                        match PlayerStream::new(stream) {
                            Ok(player_stream) => {
                                *slot = Some(player_stream);
                            }
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

    /// Waits for a move from a specific player.
    /// Returns `None` if the player disconnects instead of sending a move.
    pub fn wait_for_move(&mut self, player_index: usize) -> Option<usize> {
        if let (Some(message), _) = self.receive_message(player_index) {
            match message {
                ClientMessage::ClientMove(cell_index) => Some(cell_index),
                // If we get a StartGame message during the game, ignore it.
                ClientMessage::StartGame => {
                    eprintln!("Player {} sent StartGame during an active game. Ignoring.", player_index);
                    None
                }
            }
        } else {
            None // Player disconnected or an error occurred.
        }
    }

    pub fn send_info(&mut self, id: usize, cannot_start_game: bool) {
        self.send_message(&ServerMessage::Info(Info::new(id, self.get_connected_player_count(), if cannot_start_game {Some("Unable to start game".to_string())} else {None})), id);
    }

    /// Sends the current game state to one player
    pub fn send_game_state(&mut self, state: ClientGameState, id: usize) {
        self.send_message(&ServerMessage::GameState(state), id);
    }

    pub fn send_win_message(&mut self, state: ClientGameState, winner: usize, id: usize) {
        let mut msg = String::new();
        if id == winner {
            msg.push_str("You win!");
        } else {
            msg.push_str("You lose.")
        }

        self.send_message(&ServerMessage::GameOver(state, if id == winner {"You win".to_string()} else {"You lose".to_string()}), id);
    }

    fn send_message(&mut self, msg: &ServerMessage, id: usize) {
        if let Some(player_stream) = &mut self.player_streams[id] {
            let json = match serde_json::to_string(msg) {
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

    /// A helper to get the current number of connected players.
    pub fn get_connected_player_count(&self) -> usize {
        self.player_streams.iter().filter(|s| s.is_some()).count()
    }
}