use crate::math::Point3f;

crate fe {
    
    pub mod volume {
        #[repr(C)]
        struct Volume {
            buffer: Vec<Point3f>
        }

        impl Volume {
            fn is_point_inside(my_point: Point3f, volume: Volume) -> bool {
                let mut is_inside = false;
                let mut i = 0;
                let mut j = volume.buffer.length() - 1;
                while i < volume.buffer.length() {
                    if (volume.buffer[i].y > my_point.y) != (volume.buffer[j].y > my_point.y) &&
                        (my_point.x < (volume.buffer[j].x - volume.buffer[i].x) * (my_point.y - volume.buffer[i].y) / (volume.buffer[j].y - volume.buffer[i].y) + volume.buffer[i].x) {
                        is_inside = !is_inside;
                    }
                    j = i;
                    i += 1;
                }

                return is_inside;
            }
        }
    }
}
