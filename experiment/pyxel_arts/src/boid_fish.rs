use pyxel::{Pyxel, PyxelCallback};
use std::iter;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 192;
const FISH_COUNT: usize = 300;

#[derive(Clone)]
struct Fish {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

impl Fish {
    fn new(pyxel: &mut Pyxel) -> Self {
        Fish {
            x: pyxel.rndf(0.0, WIDTH as f64),
            y: pyxel.rndf(0.0, HEIGHT as f64),
            vx: pyxel.rndf(-1.0, 1.0),
            vy: pyxel.rndf(-1.0, 1.0),
        }
    }

    fn update(&mut self, pyxel: &mut Pyxel) {
        self.x += self.vx;
        self.y += self.vy;
        if self.x < 0.0 { self.x = 0.0; self.vx = -self.vx; }
        else if self.x > WIDTH as f64 - 4.0 { self.x = WIDTH as f64 - 4.0; self.vx = -self.vx; }
        if self.y < 0.0 { self.y = 0.0; self.vy = -self.vy; }
        else if self.y > HEIGHT as f64 - 4.0 { self.y = HEIGHT as f64 - 4.0; self.vy = -self.vy; }
        self.vx += pyxel.rndf(-0.1, 0.1);
        self.vy += pyxel.rndf(-0.1, 0.1);
        self.vx = self.vx.clamp(-1.5, 1.5);
        self.vy = self.vy.clamp(-1.5, 1.5);
    }
}

struct App {
    w: u32,
    h: u32,
    fishes: Vec<Fish>,
}


impl App {
    fn init() {
        let w = 256;
        let h = 192;

        let mut pyxel = pyxel::init(
            w,
            h,
            Some("Hello, Pyxel in Rust!"),
            None,
            None,
            None,
            None,
            None,
        );
        pyxel.mouse(true);
        pyxel.warp_mouse(10.0, 10.0);

        let fishes = (0..FISH_COUNT).map(|_| Fish::new(&mut pyxel)).collect();

        let app = App { w: w, h: h, fishes: fishes };
        pyxel.run(app);
    }
}

impl PyxelCallback for App {
    fn update(&mut self, pyxel: &mut Pyxel) {
        // if pyxel.frame_count < 60 * 6 {
        //     self.x += (pyxel.frame_count % 2) as f64;
        //     self.y += 1.0;
        // }

        if pyxel.btnp(pyxel::KEY_Q, None, None) {
            pyxel.quit();
        }

        if pyxel.btnp(pyxel::MOUSE_BUTTON_LEFT, None, None) {
            let x = pyxel.mouse_x;
            let y = pyxel.mouse_y;

            //self.node_man.rise_node(x, y, pyxel);
        }

        for fish in &mut self.fishes { fish.update(pyxel); }
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(1);
        
        for fish in &self.fishes {
            pyxel.rect(fish.x as f64, fish.y as f64, 4.0, 2.0, 7);
        }
    }
}

pub fn main() {
    App::init();
}
