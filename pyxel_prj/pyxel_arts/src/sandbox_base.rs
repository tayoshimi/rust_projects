use pyxel::{Pyxel, PyxelCallback};

pub struct App {
    x: f32,
    y: f32,
}

impl App {
    fn init() {
        let mut pyxel = pyxel::init(
            200,
            200,
            Some("Hello, Pyxel in Rust!"),
            None,
            None,
            None,
            None,
            None,
        );
        pyxel.mouse(true);
        pyxel.warp_mouse(10.0, 10.0);


        let app = App { x: 0.0, y: 0.0 };
        pyxel.run(app);
    }
}

impl PyxelCallback for App {
    fn update(&mut self, pyxel: &mut Pyxel) {
        if pyxel.frame_count < 60 * 6 {
            self.x += (pyxel.frame_count % 2) as f32;
            self.y += 1.0;
        }

        if pyxel.btnp(pyxel::KEY_Q, None, None) {
            pyxel.quit();
        }
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(3);
        pyxel.circ(self.x, self.y, 2.0, 16);
    }
}

pub fn main() {
    App::init();
}
