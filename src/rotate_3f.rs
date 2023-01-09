#[repr(C)]
pub struct Rotate3f {
    x: f32,
    y: f32,
    z: f32
}

impl Rotate3f {
    fn new(x: f32, y: f32, z: f32) -> Rotate3f {
        Rotate3f { x: x, y: y, z: z }
    }
}