
    pub mod audio_sample {
        use crate::numeric_clamp::fe::numeric_clamp;

        #[repr(C)]
        pub struct AudioSample {
            amplitude: f32,
            frequency: i32,
        }

        impl AudioSample {
            fn send_to_device(id: &str) {
                
            }

            fn set_loudness(mut self, loudness: f32) {
                self.amplitude = numeric_clamp::clamp_f32(loudness, 0.0, 1.0);
            }
    
            fn set_frequency(mut self, frequency: i32) { 
                self.frequency = frequency;
            }
        }
    }
