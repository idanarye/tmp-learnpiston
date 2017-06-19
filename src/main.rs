#![feature(macro_vis_matcher)]

extern crate piston_window;
extern crate find_folder;

#[macro_use]
extern crate learnpiston;

use piston_window::*;
use piston_window::math::*;

use learnpiston::object::Object;

pub fn main() {
    let mut game = Game::default();
    let mut window: PistonWindow = WindowSettings::new("learnpiston", game.area_size).exit_on_esc(true).build().unwrap();
    game.on_load(&mut window);
    while let Some(inp) = window.next() {
        match inp {
            Input::Update(upd) => game.on_update(upd),
            Input::Resize(width, height) => game.on_resize(width, height),
            Input::Render(ren) => game.on_draw(ren, &mut window, inp),
            inp => game.on_input(inp),
        }
    }
}

key_map! {Keys
    d_up = Up;
    d_down = Down;
    d_left = Left;
    d_right = Right;
}

super_default! {
    struct Game {
        area_size: [u32; 2] = [600, 800],
        player: Object,
        rot_speed: f64 = 0.01,
        speed: Vec2d = [0.0, 0.0],
        acl: f64 = 1000.0,
        brake: f64 = 0.4,
        abs_min: f64 = 10.0,
        keys: Keys = Keys::new(),
    }
}

impl Game {
    fn on_load(&mut self, window: &mut PistonWindow) {
        let assets_dir = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let sprite_file = assets_dir.join("xspr5.png");
        println!("{:?}", sprite_file);
        self.player.sprite = Some(Texture::from_path(
            &mut window.factory,
            &sprite_file,
            Flip::None,
            &TextureSettings::new()).unwrap());
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        self.area_size = [width, height];
    }

    fn on_input(&mut self, inp: Input) {
        self.keys.update(&inp);
    }

    fn on_update(&mut self, upd: UpdateArgs) {
        let mut acl: Vec2d = [0.0, 0.0];
        if self.keys.d_up {
            acl[1] -= self.acl;
        }
        if self.keys.d_down {
            acl[1] += self.acl;
        }

        let mut rot = 0.0;
        if self.keys.d_left {
            rot -= self.rot_speed;
            // acl[0] -= self.acl;
        }
        if self.keys.d_right {
            rot += self.rot_speed;
            // acl[0] += self.acl;
        }
        self.player.rotation += rot;

        let acl = transform_vec(rotate_radians(self.player.rotation), acl);

        for (s, a) in self.speed.iter_mut().zip(acl.iter()) {
            *s *= self.brake.powf(upd.dt);
            if *a == 0.0 && s.abs() < self.abs_min {
                *s = 0.0;
            }
            *s += upd.dt * *a;
        }

        for ((p, s), d) in self.player.pos.iter_mut().zip(self.speed.iter_mut()).zip(self.area_size.iter()) {
            if 0.0 < *s && *d as f64 <= *p + upd.dt * *s + self.player.size / 2.0 {
                *s = -*s;
            } else if *s < 0.0 && *p - upd.dt * *s - self.player.size / 2.0 <= 0.0 {
                *s = -*s;
            }
            *p += upd.dt * *s;
        }
    }
    fn on_draw<E: GenericEvent>(&self, _ren: RenderArgs, window: &mut PistonWindow, e: E) {
        window.draw_2d(&e, |c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            self.player.render(g, c.transform);
        });
    }
}
