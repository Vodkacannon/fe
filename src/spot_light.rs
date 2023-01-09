pub mod fe {
    pub mod spot_light {
        #[repr(C)]
        pub(crate) struct SpotLight {
            is_on: bool,
            is_static: bool,
            power: f32,
            radius: f32,
            softness: f32
        }

        impl SpotLight {
            
        }
    }
}