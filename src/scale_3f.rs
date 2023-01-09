#[repr(C)]
struct Scale3f {
    x: f32,
    y: f32,
    z: f32
}

impl Scale3f {
    fn new(x: f32, y: f32, z: f32) -> Scale3f {
        Scale3f { x: x, y: y, z: z }
    }

    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn get_z(&self) -> f32 {
        self.z
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    fn get(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}