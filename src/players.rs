pub mod players {
    #[repr(C)]
    pub(crate) struct Players<Player> {
        buffer: Vec<Player>
    }

    impl<Player> Players<Player> {
        pub fn new(_: Self, players: Vec<Player>) -> Self {
            Self { buffer: players }
        }

        pub fn get_players(&self) -> &Vec<Player> {
            &self.buffer
        }

        pub fn get_player(&self, index: usize) -> &Player {
            &self.buffer[index]
        }

        pub fn get_player_mut(&mut self, index: usize) -> &mut Player {
            &mut self.buffer[index]
        }

        pub fn add_player(&mut self, player: Player) {
            self.buffer.push(player);
        }

        pub fn remove_player(&mut self, index: usize) {
            self.buffer.remove(index);
        }

        pub fn remove_player_by_name(&mut self, name: String) {
            let mut index = 0;
            for player in self.buffer.iter() {
                if player.name == name {
                    self.buffer.remove(index);
                    break;
                }
                index += 1;
            }
        }

        pub fn remove_player_by_ipv6_address(&mut self, ipv6_address: String) {
            let mut index = 0;
            for player in self.buffer.iter() {
                if player.ipv6_address == ipv6_address {
                    self.buffer.remove(index);
                    break;
                }
                index += 1;
            }
        }

        pub fn get_player_by_name(&self, name: String) -> &Player {
            for player in self.buffer.iter() {
                if player.name == name {
                    return player;
                }
            }
            panic!("Player not found");
        }

        pub fn get_player_by_ipv6_address(&self, ipv6_address: String) -> &Player {
            for player in self.buffer.iter() {
                if player.ipv6_address == ipv6_address {
                    return player;
                }
            }
            panic!("Player not found");
        }

        pub fn get_player_by_name_mut(&mut self, name: String) -> &mut Player {
            for player in self.buffer.iter_mut() {
                if player.name == name {
                    return player;
                }
            }
            panic!("Player not found");
        }

        pub fn get_player_by_ipv6_address_mut(&mut self, ipv6_address: String) -> &mut Player {
            for player in self.buffer.iter_mut() {
                if player.ipv6_address == ipv6_address {
                    return player;
                }
            }
            panic!("Player not found");
        }
    }
}