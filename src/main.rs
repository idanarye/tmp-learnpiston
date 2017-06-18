#![feature(macro_vis_matcher)]

extern crate piston_window;

#[macro_use]
extern crate learnpiston;

use piston_window::*;
use piston_window::math::*;

pub fn main() {
    let mut window: PistonWindow = WindowSettings::new("learnpiston", [600, 800]).exit_on_esc(true).build().unwrap();
    let mut game = Game::new();
    while let Some(e) = window.next() {
        match e {
            Event::Update(upd) => game.on_update(upd),
            Event::Render(ren) => game.on_draw(ren, &mut window, e),
            Event::Input(inp) => game.on_input(inp),
            _ => {}
        }
    }
}

key_map! {Keys
    d_up = Up;
    d_down = Down;
    d_left = Left;
    d_right = Right;
}

initialized_object! {
    struct Game {
        rotation: f64 = 0.0,
        size: f64 = 100.0,
        pos: Vec2d = [100.0, 100.0],
        speed: Vec2d = [0.0, 0.0],
        acl: f64 = 1000.0,
        brake: f64 = 0.4,
        abs_min: f64 = 10.0,
        keys: Keys = Keys::new(),
    }
}

impl Game {
    fn on_input(&mut self, inp: Input) {
        self.keys.update(&inp);
    }
    fn on_update(&mut self, upd: UpdateArgs) {
        self.rotation += upd.dt;

        let mut acl: Vec2d = [0.0, 0.0];
        if self.keys.d_up {
            acl[1] -= self.acl;
        }
        if self.keys.d_down {
            acl[1] += self.acl;
        }
        if self.keys.d_left {
            acl[0] -= self.acl;
        }
        if self.keys.d_right {
            acl[0] += self.acl;
        }

        for (s, a) in self.speed.iter_mut().zip(acl.iter()) {
            *s *= self.brake.powf(upd.dt);
            if *a == 0.0 && s.abs() < self.abs_min {
                *s = 0.0;
            }
            *s += upd.dt * *a;
            /*
            let brake_mgn = upd.dt * self.brake;
            if 0.0 < *s {
                if *s < brake_mgn {
                    *s = 0.0;
                } else {
                    *s -= brake_mgn;
                }
            } else {
                if -*s < brake_mgn {
                    *s = 0.0;
                } else {
                    *s += brake_mgn;
                }
            }
            */
        }

        for ((p, s), d) in self.pos.iter_mut().zip(self.speed.iter_mut()).zip([600.0, 800.0].iter()) {
            if 0.0 < *s && *d <= *p + upd.dt * *s + self.size / 2.0 {
                *s = -*s;
            } else if *s < 0.0 && *p - upd.dt * *s - self.size / 2.0 <= 0.0 {
                *s = -*s;
            }
            *p += upd.dt * *s;
        }
        // self.pos[0] += 1.0;
    }
    fn on_draw<E: GenericEvent>(&self, _ren: RenderArgs, window: &mut PistonWindow, e: E) {
        window.draw_2d(&e, |c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            let center = c.transform.trans(self.pos[0], self.pos[1]);
            let square = rectangle::square(0.0, 0.0, self.size);
            let red = [1.0, 0.0, 0.0, 1.0];
            rectangle(red, square, center.rot_rad(self.rotation).trans(-50.0, -50.0), g);
        });
    }
}
