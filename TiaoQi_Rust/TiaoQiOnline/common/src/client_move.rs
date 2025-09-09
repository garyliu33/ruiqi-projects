use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClientMove {
    pub cell: usize
}

impl ClientMove {
    pub fn new(cell: usize) -> Self {
        Self { cell }
    }
}