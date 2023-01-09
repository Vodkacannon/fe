crate fe {
    pub mod volume {
        #[repr(C)]
        struct Volume {
            buffer: Vec<Point3f>
        }

        impl Volume {
            fn is_point_inside(volume: Volume) -> bool {

            }
        }
    }
}
