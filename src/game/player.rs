use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Player {
    pub id: u128,
    pub ready: bool,
}

impl Player {
    pub fn new(id: u128) -> Self {
        Player {
            id,
            ready: false,
        }
    }
    pub fn set_ready(&mut self, ready: bool) -> bool {
        self.ready = ready;
        self.ready
    }
}
