
    
        #[repr(C)]
        pub struct Rgb {
            r: i16,
            g: i16,
            b: i16
        }

        impl Rgb {
            pub fn black() -> Rgb {
                Rgb { r: 0, g: 0, b: 0 }
            }
        
            pub fn white() -> Rgb {
                Rgb { r: 255, g: 255, b: 255 }
            }
        
            // Another associated function, taking two arguments:
            pub fn new(r: i16, g: i16, b: i16) -> Rgb {
                Rgb { r, g, b }
            }

            pub fn to_str(self) -> String {
                return self.r.to_string() + ", " + &self.g.to_string() + ", " + &self.b.to_string();
            }
        }
    
