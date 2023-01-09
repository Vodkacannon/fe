pub mod fe {
    pub mod spot_lights {
        #[repr(C)]
        pub(crate) struct SpotLights<SpotLight> {
            is_on: bool,
            is_static: bool,
            power: f32,
            radius: f32,
            softness: f32,
            buffer: Vec<SpotLight>
        }

        impl<SpotLight> SpotLights<SpotLight> {
            pub(crate) fn new(is_on: bool, is_static: bool, power: f32, radius: f32, softness: f32, buffer: Vec<SpotLight>) -> Self {
                Self { is_on, is_static, power, radius, softness, buffer } 
            }
        }
    }
}