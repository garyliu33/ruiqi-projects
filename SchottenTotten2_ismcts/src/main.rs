mod game_state;
mod node;
mod parallel_multi_observer_ismcts;
mod schotten_totten_2_state;

use parallel_multi_observer_ismcts::parallel_multi_observer_ismcts;
use prost::Message;
use prost::bytes::{BufMut, BytesMut};
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

use crate::schotten_totten_2_state::{SchottenTotten2State, com_st_proto};

// Helper function to write a length-delimited protobuf message to a stream
fn write_delimited<T: Message>(stream: &mut TcpStream, msg: &T) -> io::Result<()> {
    // Encode the message to a buffer
    let mut prefixed_buf = Vec::new();
    prost::Message::encode_length_delimited(msg, &mut prefixed_buf)?;

    // Write the full length-prefixed buffer to the stream
    stream.write_all(&prefixed_buf)?;

    Ok(())
}

// Helper function to read a length-delimited protobuf message from a stream
fn read_delimited<T: Message + Default>(stream: &mut TcpStream) -> io::Result<T> {
    // Read the varint length prefix into a temporary buffer.
    let mut len_buf = BytesMut::with_capacity(10); // A varint can be up to 10 bytes long.
    let mut message_len: usize = 0;
    let mut bytes_read_for_len = 0;

    loop {
        // Ensure we have capacity for the next byte.
        if len_buf.remaining_mut() == 0 {
            len_buf.reserve(1);
        }

        // Read one byte at a time until the varint is complete.
        let mut byte = [0u8; 1];
        stream.read_exact(&mut byte)?;
        len_buf.put_slice(&byte);
        bytes_read_for_len += 1;

        // Try to decode the length prefix from the buffer.
        let mut temp_buf = len_buf.clone().freeze();
        if let Ok(len) = prost::decode_length_delimiter(&mut temp_buf) {
            message_len = len;
            break;
        } else if bytes_read_for_len >= 10 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Varint too large or invalid",
            ));
        }
    }

    // Read the actual message bytes
    let mut msg_buf = BytesMut::with_capacity(message_len);
    msg_buf.resize(message_len, 0);
    stream.read_exact(&mut msg_buf)?;

    // Decode the message
    Ok(T::decode(msg_buf.freeze()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?)
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:12345").unwrap();

    loop {
        let game_state_proto = read_delimited::<com_st_proto::GameStateProto>(&mut stream).unwrap();
        let schotten_totten_2_state = SchottenTotten2State::from_proto(&game_state_proto);
        if !schotten_totten_2_state.is_client_turn {
            continue;
        }
        let m = parallel_multi_observer_ismcts(&schotten_totten_2_state, 10000, 10);
        let move_proto = m.to_proto();
        write_delimited(&mut stream, &move_proto).unwrap();
    }
}
