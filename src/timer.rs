pub mod timer {
    use std::{ thread, time::Duration };

    //Timer always starts at zero.
    struct Timer {
        is_stopped: bool,
        current_time_ms: i32,
    }

    impl Timer {
        fn start() {
            while !is_stopped {
                thread::sleep(Duration::from_millis(1));
                current_time_ms += 1;
            }
        }

        fn stop() {
            is_stopped = true;
        }

        fn reset() {
            current_time_ms = 0;
        }
    }
}
