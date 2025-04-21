use pyxel::{Pyxel, PyxelCallback};
use std::iter;
use std::time::Instant;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 192;
const OBJECT_COUNT: usize = 300;

#[derive(Clone)]
struct Object {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    color: u8,
}

impl Object {
    fn new() -> Self {
        Object {
            x: Pyxel::rndf(0.0, WIDTH as f64),
            y: Pyxel::rndf(0.0, HEIGHT as f64),
            vx: Pyxel::rndf(-1.0, 1.0),
            vy: Pyxel::rndf(-1.0, 1.0),
            color: Pyxel::rndi(4, 11) as u8,
        }
    }

    fn update(&mut self, pyxel: &mut Pyxel) {
        self.x += self.vx;
        self.y += self.vy;
        if self.x < 0.0 { self.x = 0.0; self.vx = -self.vx; }
        else if self.x > WIDTH as f64 - 4.0 { self.x = WIDTH as f64 - 4.0; self.vx = -self.vx; }
        if self.y < 0.0 { self.y = 0.0; self.vy = -self.vy; }
        else if self.y > HEIGHT as f64 - 4.0 { self.y = HEIGHT as f64 - 4.0; self.vy = -self.vy; }
        self.vx += Pyxel::rndf(-0.1, 0.1);
        self.vy += Pyxel::rndf(-0.1, 0.1);
        self.vx = self.vx.clamp(-1.5, 1.5);
        self.vy = self.vy.clamp(-1.5, 1.5);
    }
}

pub struct App {
    w: u32,
    h: u32,
    objects: Vec<Object>,
    start_time: Instant, // FPS計算用の開始時間
    prev_frame_count: u32, // 前回のフレームカウント
    fps: f64, // 計算されたFPS
}


impl App {
    pub fn init(pyxel: &mut Pyxel) -> Self {
        pyxel.mouse(true);
        pyxel.warp_mouse(10.0, 10.0);

        let objects = (0..OBJECT_COUNT).map(|_| Object::new()).collect();

        let start_time = Instant::now();
        let prev_frame_count = pyxel.frame_count;
        let fps = 0.0;

        App { w:WIDTH, h:HEIGHT, objects, start_time, prev_frame_count, fps }
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
        }

        for object in &mut self.objects { object.update(pyxel); }

        // FPSの計算（1秒ごとに更新）
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed >= 1.0 {
            let frames = pyxel.frame_count - self.prev_frame_count;
            self.fps = frames as f64 / elapsed;
            self.start_time = Instant::now();
            self.prev_frame_count = pyxel.frame_count;
        }
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(1);
        
        for object in &self.objects {
            pyxel.rect(object.x as f64, object.y as f64, 4.0, 2.0, object.color);
        }

        // FPSの表示
        let fps_text = format!("FPS: {:.2}", self.fps);
        pyxel.text(self.w as f64 - 50.0, 10.0, &fps_text, 7, None);
    }
}

pub fn main() {
    let mut pyxel = pyxel::init(
        WIDTH,
        HEIGHT,
        Some("Hello, Pyxel in Rust!"),
        None,
        None,
        None,
        None,
        None,
    );
    let app = App::init(&mut pyxel);
    pyxel.run(app);
}
