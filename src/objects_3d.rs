pub mod fe {
    pub mod objects_3d {
        #[repr(C)]
        pub(crate) struct Objects3D<Object3D> {
            should_render: bool,
            buffer: Vec<Object3D>
        }

        impl<Object3D> Objects3D<Object3D> {
            fn new(should_render: bool, buffer: Vec<Object3D>) -> Self { Self { should_render, buffer } }
        }
    }
}