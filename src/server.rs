use crate::{players::players::Players, player::Player};

pub struct Server {
    //String for now.
    host_ip: String,
    host_port: i32,
    max_players: i32,
    players: Players<Player>,
    password: String
}

impl Server {
    fn Server(host_ip: String, host_port: i32, max_players: i32, players: Players<Player>, password: String) -> Self {
        Self { host_ip, host_port, max_players, players, password }
    }

    fn start() {

    }

    fn stop() {

    }

    fn get_packet_from_player() {

    }

    fn send_packet_to_player() {

    }

    
}