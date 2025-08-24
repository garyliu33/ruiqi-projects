mod network;
mod player;
mod game;

use crate::network::start_server;

fn main() -> std::io::Result<()> {
    start_server()?;
    Ok(())
}
