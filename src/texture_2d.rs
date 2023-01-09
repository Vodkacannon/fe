use crate::rgba::Rgba;

#[repr(C)]
pub struct Texture2D<Rgba> {
    render: bool,
    buffer: Vec<Vec<Rgba>>
}

impl Texture2D<Rgba> {
    pub fn new(render: bool) -> Texture2D<Rgba> {
        Texture2D { render, buffer: todo!() }
    }

    fn transparency(&self, amount: i16) {
        for u in 0..self.buffer[0].len() {
            for v in 0..self.buffer[1].len() {
                //self.buffer[u][v] = amount
            }
        }
    }

    fn brightness(&self, amount: i32) {
        for u in 0..self.buffer[0].len() {
            for v in 0..self.buffer[1].len() {
                //self.buffer[u][v] = amount
            }
        }
    }
}       