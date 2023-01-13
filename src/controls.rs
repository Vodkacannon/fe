struct Controls {
    are_enabled: bool,
    lock_if_opposite_directions_pressed: bool,
    forward: f32,
    backward: f32,
    left: f32,
    right: f32,
    //Are we in, space/water/sky?
    up: f32,
    down: f32
}

impl Controls {
    fn is_forward_pressed() -> bool {
        
    }
    
    fn is_backward_pressed() -> bool {
    
    }
    
    fn is_forward_and_backward_pressed() -> bool {
        return is_forward_pressed() && is_backward_pressed();
    }
    
    fn is_left_pressed() -> bool {
        
    }

    fn is_right_pressed() -> bool {
    
    }

    fn is_left_and_right_pressed() -> bool {
        return is_left_pressed() && is_right_pressed();
    }
    
    fn is_up_pressed() -> bool {
    
    }
    
    fn is_down_pressed() -> bool {
    
    }
    
    fn is_up_and_down_pressed() -> bool {
        return is_up_pressed() && is_down_pressed();
    }
}
