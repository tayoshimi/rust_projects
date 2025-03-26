use pyxel::{Pyxel, PyxelCallback};
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
    fn new(pyxel: &mut Pyxel) -> Self {
        Object {
            x: pyxel.rndf(0.0, WIDTH as f64),
            y: pyxel.rndf(0.0, HEIGHT as f64),
            vx: pyxel.rndf(-1.0, 1.0),
            vy: pyxel.rndf(-1.0, 1.0),
            color: pyxel.rndi(4, 11) as u8,
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

struct PianoKey {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    key_type: char,  // 'C', 'D', 'E', 'F', 'G', 'A', 'B'
}

impl PianoKey {
    fn new(x: f64, y: f64, width: f64, height: f64, key_type: char) -> Self {
        PianoKey { x, y, width, height, key_type }
    }

    fn draw(&self, pyxel: &mut Pyxel) {
        pyxel.rect(self.x, self.y, self.width, self.height, 7);
        pyxel.rectb(self.x, self.y, self.width, self.height, 0);
        // Draw the note name...
    }

    fn play_sound(&self, pyxel: &mut Pyxel) {
        match self.key_type {
            'C' => pyxel.play1(1, 0, Some(0), false, true),
            'D' => pyxel.play1(1, 1, Some(0), false, true),
            'E' => pyxel.play1(1, 2, Some(0), false, true),
            'F' => pyxel.play1(1, 3, Some(0), false, true),
            'G' => pyxel.play1(1, 4, Some(0), false, true),
            'A' => pyxel.play1(1, 5, Some(0), false, true),
            'B' => pyxel.play1(1, 6, Some(0), false, true),
            _ => {}
        }
    }
}

struct Piano {
    keys: Vec<PianoKey>,
}

impl Piano {
    fn new() -> Self {
        let mut keys = Vec::new();
        
        // Define the 2 octaves of piano keys
        for i in 0..12 * 2 {  // 7 notes per octave * 2 octaves
            let x = (i as f64) * (WIDTH as f64 / (12 * 2) as f64);
            let y = HEIGHT as f64 - 50.0;
            let width = WIDTH as f64 / (12 * 2) as f64;
            let height = 40.0;
            keys.push(PianoKey::new(x, y, width, height, ['C', 'D', 'E', 'F', 'G', 'A', 'B'][i % 7]));
        }
        
        Piano { keys }
    }

    fn draw(&self, pyxel: &mut Pyxel) {
        for key in &self.keys {
            key.draw(pyxel);
        }
    }

    fn play_sound(&self, x: f64, pyxel: &mut Pyxel) {
        let mut found = false;
        for key in &self.keys {
            if key.x <= x && x < key.x + key.width {
                key.play_sound(pyxel);
                found = true;
                break;
            }
        }
        if !found {
            //pyxel.play(0, Some(1), false, None);  // Play a default sound if no key is pressed
        }
    }
}

struct App {
    w: u32,
    h: u32,
    piano: Piano,
    objects: Vec<Object>,
    start_time: Instant, // FPS計算用の開始時間
    prev_frame_count: u32, // 前回のフレームカウント
    fps: f64, // 計算されたFPS
}


impl App {
    fn init() {
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
        pyxel.mouse(true);
        pyxel.warp_mouse(10.0, 10.0);

        //pyxel.load("assets.png");
        //pyxel.play(0, Some(0), None, true, false);

        let piano = Piano::new();

        let objects = (0..OBJECT_COUNT).map(|_| Object::new(&mut pyxel)).collect();

        let start_time = Instant::now();
        let prev_frame_count = pyxel.frame_count;
        let fps = 0.0;

        let app = App { w:WIDTH, h:HEIGHT, piano, objects, start_time, prev_frame_count, fps };
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
            self.piano.play_sound(x as f64, pyxel);
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
        
        self.piano.draw(pyxel);

        for object in &self.objects {
            pyxel.rect(object.x as f64, object.y as f64, 4.0, 2.0, object.color);
        }

        // FPSの表示
        let fps_text = format!("FPS: {:.2}", self.fps);
        pyxel.text(self.w as f64 - 50.0, 10.0, &fps_text, 7, None);
    }
}

pub fn main() {
    App::init();
}
