//Possibly change to Vec2f. 
//Maybe use doubles for all float types.
#[repr(C)]
struct Point2f {
    x: f32,
    y: f32
}

impl Point2f {
    fn origin() -> Point2f {
        Point2f { x: 0.0, y: 0.0 }
    }
  
    // Another associated function, taking two arguments:
    fn new(x: f32, y: f32) -> Point2f {
        Point2f { x: x, y: y }
    }
}