extern crate piston_window;

// use piston_window::*;
use piston_window::math::*;

initialized_object! {
    pub struct Object {
        pos: Vec2d = [0.0, 0.0],
    }
}

#[allow(dead_code)]
impl Object {
    pub fn mov(&mut self, offset: Vec2d) {
        for (p, o) in self.pos.iter_mut().zip(offset.iter()) {
            *p += *o;
        }
    }
}
