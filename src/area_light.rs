pub mod fe {
    pub mod area_light {
        #[repr(C)]
        pub struct AreaLight {
            is_on: bool,
            is_static: bool,
            power: f32,
            radius: f32,
            softness: f32
        }

        impl AreaLight {
            pub fn new(is_on: bool, is_static: bool, power: f32, radius: f32, softness: f32) -> Self {
                 Self { is_on, is_static, power, radius, softness } 
            }
        }
    }
}