
    
        #[repr(C)]
        pub struct Player {
            pub(crate) ipv6_address: String,
            pub(crate) is_alive: bool,
            pub(crate) name: String
        }

        impl Player {
            pub fn new(ipv6_address: String, is_alive: bool, name: String) -> Self {
                Self { ipv6_address, is_alive, name } 
            }
        }
    
