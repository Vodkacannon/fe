
    pub mod players {
        #[repr(C)]
        pub(crate) struct Players<Player> {
            buffer: Vec<Player>
        }

        impl<Player> Players<Player> {
            pub fn new(_: Self, players: Vec<Player>) -> Self {
                Self { buffer: players }
            }
        }

        
    }
