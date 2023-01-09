use crate::player::{Player, self};

pub struct LocalPlayer {
    player: Player
}

impl LocalPlayer {
    fn new(mut self) {
        self.player = player::Player { ipv6_address: "".to_string(), is_alive: false, name: "[Name]".to_string() };
    }
}