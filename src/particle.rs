
    pub mod particle {
        use crate::{point_3f::Point3f, texture_2d::Texture2D, rgba::Rgba};

        #[repr(C)]
        struct Particle {
            position: Point3f,
            //Should each particle have a render flag?
            should_render: bool,
            life_time_ms: i16,
            scale: f32,
            texture: Texture2D<Rgba>
        }

        impl Particle {
            fn update_life_time(mut self) {
                if self.life_time_ms > 0 {
                    self.life_time_ms -= 1;
                } else {
                    self.should_render = false;
                }
            }
        }
    }
