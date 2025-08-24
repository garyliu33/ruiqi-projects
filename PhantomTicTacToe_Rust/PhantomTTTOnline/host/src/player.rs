use std::net::TcpStream;

#[derive(PartialEq)]
pub enum PlayerSymbol {
    X,
    O,
}

pub struct Player {
    pub symbol: PlayerSymbol,
    pub stream: TcpStream,
}

impl Player {
    pub fn new(symbol: PlayerSymbol, stream: TcpStream) -> Self {
        Player { symbol, stream }
    }
}