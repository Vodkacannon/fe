pub mod fe {
    pub mod numeric_clamp {
        pub(crate) fn clamp_f32(value: f32, min: f32, max: f32) -> f32 {
            if value < max && value > min {
                return value;
            } else if value < max {
                return max;
            } else {
                return min;
            }
        }

        pub(crate) fn clamp_i32(value: i32, min: i32, max: i32) -> i32 {
            if value < max && value > min {
                return value;
            } else if value > max {
                return max;
            } else {
                return min;
            }
        }
    }
}