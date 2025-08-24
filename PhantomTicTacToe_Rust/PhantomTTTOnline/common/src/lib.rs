use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub content: String,
    pub get_input: bool,
}

impl Message {
    pub fn new(content: &str, get_input: bool) -> Self {
        Message { content: content.to_string(), get_input }
    }
}