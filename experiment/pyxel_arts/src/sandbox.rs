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
        // if pyxel.frame_count < 60 * 6 {
        //     self.x += (pyxel.frame_count % 2) as f64;
        //     self.y += 1.0;
        // }

        if pyxel.btnp(pyxel::KEY_Q, None, None) {
            pyxel.quit();
        }

        if pyxel.btnp(pyxel::MOUSE_BUTTON_LEFT, None, None) {
            let x = pyxel.mouse_x - 40;
            let y = pyxel.mouse_y - 40;
        }
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        /*
        pyxel.cls(1)

        pyxel.line(0, H/2, W, H/2, pyxel.COLOR_WHITE)
        for c in range(360):
          x = int(W * c / 360)
          y = H/2 + pyxel.sin(c) * 50
          pyxel.pset(x, y, pyxel.COLOR_RED)
          */

        pyxel.cls(1);
        //pyxel.text(10.0, 20.0, &format!("Player {}'s turn!", self.player_turn), 10, None);
        //pyxel.rect(40.0, 40.0, 120.0, 80.0, 16);

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
