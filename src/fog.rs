pub mod fe {
    pub mod fog {
        pub(crate) struct Fog {
            color: Rgba,
            softness: f32,
        }

        impl Fog {
            fn new(&mut self) {
                self.color = Rgba::new(0, 0, 0, 0);
            }
        }
    }
}