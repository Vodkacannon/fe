use crate::point_3f::Point3f;

pub struct Force {
    mass: f32,
    acceleration: Point3f
}

impl Force {
    fn get_magnitude(&self) -> f32 {
        return self.mass * self.acceleration.get_length()
    }
}