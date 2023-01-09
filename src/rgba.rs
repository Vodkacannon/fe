
    #[repr(C)]
    pub struct Rgba {
        r: i16,
        g: i16,
        b: i16,
        a: i16
    }
    
    impl Rgba {
        fn black() -> Rgba {
            Rgba { r: 0, g: 0, b: 0, a: 0 }
        }
      
        fn white() -> Rgba {
            Rgba { r: 255, g: 255, b: 255, a: 0 }
        }
      
        // Another associated function, taking two arguments:
        fn new(r: i16, g: i16, b: i16, a: i16) -> Rgba {
            Rgba { r, g, b, a }
        }
    }

