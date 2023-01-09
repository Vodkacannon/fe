use crate::point_3f::Point3f;

#[repr(C)]
struct Animation {
    looped: bool,
    time_scale: f32,
    buffer: Vec<Point3f>
}

impl Animation {
    fn new(looped: bool, time_scale: f32, buffer: Vec<Point3f>) -> Self {
        Self { looped, time_scale, buffer }
    }
}
