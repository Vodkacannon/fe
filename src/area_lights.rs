pub mod fe {
    pub(crate) mod area_lights {
        #[repr(C)]
        pub(crate) struct AreaLights<AreaLight> {
            buffer: Vec<AreaLight>
        }

        impl<AreaLight> AreaLights<AreaLight> {
            pub(crate) fn new(buffer: Vec<AreaLight>) -> Self {
                 Self { buffer } 
            }
        }
    }
}