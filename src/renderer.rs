//Vulkan, OpenGL, or Direct3D?

use crate::camera::Camera;

pub struct Renderer {
    is_frame_drawn: bool,
    is_running: bool,
    //Possibly use mulitple threads?
    thread_count: i16,
    width: i32,
    height: i32,
    pixel_count: i32,
    frame_dt_ms: i32,
    target_fps: i32,
    frame_first_t_ms: i32,
    frame_second_t_ms: i32,
    current_fps: f32,
    local_camera: Camera,
    visible_items: i32,
}

impl Renderer {
    fn update_pixel_count(&mut self) {
        self.pixel_count = self.width * self.height;
    }

    fn update_frame_dt(&mut self) {
        self.frame_dt_ms = self.frame_second_t_ms - self.frame_first_t_ms;
    }

    fn update_current_fps(&mut self) {
        unimplemented!("todo.")
    }

    fn draw_fog() {
        unimplemented!("todo.")
    }

    fn render_loop() {
        unimplemented!("todo.")
    }
}
