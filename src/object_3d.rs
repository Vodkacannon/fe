
        use crate::{point_3f::Point3f, texture_2d::Texture2D};

        #[repr(C)]  
        pub(crate) struct Object3D<Rgba> {
            render: bool,
            is_physical: bool,
            has_physics: bool,
            vertices: Vec<Point3f>,
            texture: Texture2D<Rgba>
        }

        impl<Rgba> Object3D<Rgba> {
            fn will_render(&self) -> bool {
                return self.render;
            }
        }