pub mod area {
    #[repr(C)]
    struct Area<Point2f> {
        buffer: Vec<Point2f>
    }

    impl<Point2f> Area<Point2f> {
        fn is_point_inside() -> bool {
            return false;
        }

        fn get_volume() -> f32 {
            return 0.0;
        }
    }
}