
    pub mod global_light {
        use crate::rgb::{Rgb};

        #[repr(C)]
        pub(crate) struct GlobalLight {
            is_on: bool,
            is_static: bool,
            power: f32,
            color: Rgb
        }

        impl GlobalLight {
            pub(crate) fn new(is_on: bool, is_static: bool, power: f32, color: Rgb) -> Self { Self { is_on, is_static, power, color } }
        }
    }
