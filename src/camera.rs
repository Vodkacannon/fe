
    use crate::point_3f::Point3f;

    pub struct Camera {
        fov: i32,
        near_clip: f32,
        far_clip: f32,
        width: i32,
        height: i32,
        position: Point3f,
        direction: Point3f,
    }

    impl Camera {
        pub fn new(
            fov: i32,
            near_clip: f32,
            far_clip: f32,
            width: i32,
            height: i32,
            position: Point3f,
            direction: Point3f,
        ) -> Camera {
            Camera {
                fov: fov,
                near_clip: near_clip,
                far_clip: far_clip,
                width: width,
                height: height,
                position: position,
                direction: direction,
            }
        }
    }