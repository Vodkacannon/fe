#[repr(C)]
pub struct Point3f {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3f {
    fn origin() -> Point3f {
        Point3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    // Another associated function, taking two arguments:
    fn new(x: f32, y: f32, z: f32) -> Point3f {
        Point3f { x, y, z }
    }

    pub fn get_length(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }

    fn add(&mut self, point: Point3f) {
        self.x += point.x;
        self.y += point.y;
        self.z += point.z;
    }

    fn negate(&mut self) {
        self.x *= -1.0;
        self.y *= -1.0;
        self.z *= -1.0;
    }

    fn sub(&mut self, point: Point3f) {
        self.x -= point.x;
        self.y -= point.y;
        self.z -= point.z;
    }

    fn cross(&mut self, point: Point3f) {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        self.x = y * point.z - z * point.y;
        self.y = z * point.x - x * point.z;
        self.z = x * point.y - y * point.x;
    }
}
