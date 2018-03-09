extern crate piston_window;

use piston_window::*;

// use piston_window::*;
use piston_window::math::*;

use gfx_device_gl::{Resources, /*CommandBuffer*/};
use gfx_graphics::GfxGraphics;

super_default! {
    pub struct Object {
        pub pos: Vec2d = [100.0, 100.0],
        pub size: f64 = 100.0,
        pub rotation: f64 = 0.0,
        pub sprite: Option<Texture<Resources>>,
    }
}

#[allow(dead_code)]
impl Object {
    pub fn mov(&mut self, offset: Vec2d) {
        for (p, o) in self.pos.iter_mut().zip(offset.iter()) {
            *p += *o;
        }
    }

    pub fn mov_to(&mut self, new_pos: Vec2d) {
        for (p, np) in self.pos.iter_mut().zip(new_pos.iter()) {
            *p = *np;
        }
    }

    pub fn render<G: Graphics>(&self, g: &mut G, view: math::Matrix2d)
        where G: Graphics<Texture=Texture<Resources>>
    {
        let center = view.trans(self.pos[0], self.pos[1]).rot_rad(self.rotation).trans(-50.0, -50.0);
        if let Some(ref sprite) = self.sprite {
            image(sprite, center, g);
        } else {
            let square = rectangle::square(0.0, 0.0, self.size);
            let red = [1.0, 0.0, 0.0, 1.0];
            rectangle(red, square, center, g);
        }
    }
}
