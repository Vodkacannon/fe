pub mod fe {
    pub mod physics {
        //Renderer and physics run separately.
        struct Physics {
            //Possibly use multiple threads?
            thread_count: i16,
            time_scale: f32,
            dt: f32,
            gravity: f32
        }

        //This should be fun...
        impl Physics {
            fn integrate() {
                
            }
        }
    }
}