
pub mod sound {
    #[repr(C)]
    pub struct Sound {
        is_playing: bool,
        is_looped: bool,
        distance: f32,
        samples: i32,
        position: Point3f,
        buffer: Vec<AudioSample>
    }

    type Sounds = Vec<Sound>;

    impl Sound {
        fn update_falloff(&mut self, distance: f32) -> None {
            if distance == 0 {
                self.set_loudness(1.0);
            } else {
                self.loudness = 1.0 / (self.distance * self.distance);
            }
        }

        fn set_loudness_track(&self, loudness: f32) {
            for i in 0..self.buffer.length() {
                self.buffer[i] *= clamp_f32(loudness, 0.0, 1.0);
            }
        }

        fn set_loudness_sample(&self, sample_index: i32, loudness: f32) {
            self.buffer[sample_index] *= clamp_f32(loudness, 0.0, 1.0);
        }

        fn set_loudness(&self, loudness: f32) {
            for i in 0..self.buffer.length() {
                self.buffer[i] *= clamp_f32(loudness, 0.0, 1.0);
            }
        }

        fn get_loudness(&self) -> f32 {
            let mut loudness = 0.0;
            for i in 0..self.buffer.length() {
                loudness += self.buffer[i];
            }
            loudness / self.buffer.length()
        }
    }
}