use pyxel::{Pyxel, PyxelCallback};

pub struct App {
    w: u32,
    h: u32,
}

impl App {
    fn init() {
        let w = 160;
        let h = 120;

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


        let app = App { w: w, h: h };
        pyxel.run(app);
    }
}

impl PyxelCallback for App {
    fn update(&mut self, pyxel: &mut Pyxel) {
        if pyxel.btnp(pyxel::KEY_Q, None, None) {
            pyxel.quit();
        }
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(1);

        pyxel.line(0.0, (self.h/2) as f64, self.w as f64, (self.h/2) as f64, pyxel::COLOR_WHITE);
        for c in 0..=360 {
          let x = (self.w * c / 360) as f64;
          let y = ((self.h/2) as f64 + pyxel.sin(c as f64) * 50.0) as f64;
          pyxel.pset(x, y, pyxel::COLOR_RED);
        }
    }
}

pub fn main() {
    App::init();
}
