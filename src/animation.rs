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

    fn get_buffer(&self) -> &Vec<Point3f> {
        &self.buffer
    }

    fn get_point(&self, index: usize) -> &Point3f {
        &self.buffer[index]
    }

    fn get_point_mut(&mut self, index: usize) -> &mut Point3f {
        &mut self.buffer[index]
    }

    fn add_point(&mut self, point: Point3f) {
        self.buffer.push(point);
    }

    fn remove_point(&mut self, index: usize) {
        self.buffer.remove(index);
    }
}
