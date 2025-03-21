use pyxel::{Pyxel, PyxelCallback};
use std::f64::consts::PI;
use std::time::Instant;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 384;
const FISH_COUNT: usize = 500;

const PERCEPTION_RADIUS: f64 = 10.0; // 知覚半径
const SEPARATION_WEIGHT: f64 = 2.5;  // 分離の重み
const ALIGNMENT_WEIGHT: f64 = 1.0;   // 整列の重み
const COHESION_WEIGHT: f64 = 0.2;    // 結合の重み
const MAX_SPEED: f64 = 1.5;          // 最大速度

#[derive(Clone)]
struct Fish {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    color: u8,
    perception_radius: f64,
}

impl Fish {
    fn new(pyxel: &mut Pyxel) -> Self {
        Fish {
            x: pyxel.rndf(0.0, WIDTH as f64),
            y: pyxel.rndf(0.0, HEIGHT as f64),
            vx: pyxel.rndf(-1.0, 1.0),
            vy: pyxel.rndf(-1.0, 1.0),
            color: pyxel.rndi(3,11) as u8,
            perception_radius: PERCEPTION_RADIUS,
        }
    }

    fn get_neighbors<'a>(&self, fishes: &'a [Fish]) -> Vec<&'a Fish> {
        fishes.iter().filter(|&f| {
            let dx = self.x - f.x;
            let dy = self.y - f.y;
            let distance = (dx * dx + dy * dy).sqrt();
            distance < self.perception_radius && distance > 0.0
        }).collect()
    }

    fn separation(&self, neighbors: &[&Fish]) -> (f64, f64) {
        let mut steer_x = 0.0;
        let mut steer_y = 0.0;
        let mut count = 0;

        for neighbor in neighbors {
            let dx = self.x - neighbor.x;
            let dy = self.y - neighbor.y;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance > 0.0 {
                steer_x += dx / distance;
                steer_y += dy / distance;
                count += 1;
            }
        }

        if count > 0 {
            steer_x /= count as f64;
            steer_y /= count as f64;
        }

        (steer_x, steer_y)
    }

    fn alignment(&self, neighbors: &[&Fish]) -> (f64, f64) {
        let mut avg_vx = 0.0;
        let mut avg_vy = 0.0;
        let count = neighbors.len();

        if count > 0 {
            for neighbor in neighbors {
                avg_vx += neighbor.vx;
                avg_vy += neighbor.vy;
            }
            avg_vx /= count as f64;
            avg_vy /= count as f64;
        }

        (avg_vx, avg_vy)
    }

    fn cohesion(&self, neighbors: &[&Fish]) -> (f64, f64) {
        let mut center_x = 0.0;
        let mut center_y = 0.0;
        let count = neighbors.len();

        if count > 0 {
            for neighbor in neighbors {
                center_x += neighbor.x;
                center_y += neighbor.y;
            }
            center_x /= count as f64;
            center_y /= count as f64;

            let steer_x = center_x - self.x;
            let steer_y = center_y - self.y;
            return (steer_x, steer_y);
        }

        (0.0, 0.0)
    }

    fn update(&mut self, fishes: &[Fish]) {
        let neighbors = self.get_neighbors(fishes);

        let (sep_x, sep_y) = self.separation(&neighbors);
        let (align_x, align_y) = self.alignment(&neighbors);
        let (coh_x, coh_y) = self.cohesion(&neighbors);

        let separation_weight = SEPARATION_WEIGHT;
        let alignment_weight = ALIGNMENT_WEIGHT;
        let cohesion_weight = COHESION_WEIGHT;

        self.vx += sep_x * separation_weight + align_x * alignment_weight + coh_x * cohesion_weight;
        self.vy += sep_y * separation_weight + align_y * alignment_weight + coh_y * cohesion_weight;

        let speed = (self.vx * self.vx + self.vy * self.vy).sqrt();
        let max_speed = MAX_SPEED;
        if speed > max_speed {
            self.vx = (self.vx / speed) * max_speed;
            self.vy = (self.vy / speed) * max_speed;
        }

        self.x += self.vx;
        self.y += self.vy;

        if self.x < 0.0 { self.x = 0.0; self.vx = -self.vx; }
        else if self.x > WIDTH as f64 - 4.0 { self.x = WIDTH as f64 - 4.0; self.vx = -self.vx; }
        if self.y < 0.0 { self.y = 0.0; self.vy = -self.vy; }
        else if self.y > HEIGHT as f64 - 4.0 { self.y = HEIGHT as f64 - 4.0; self.vy = -self.vy; }
    }
}

struct App {
    w: u32,
    h: u32,
    fishes: Vec<Fish>,
    start_time: Instant, // FPS計算用の開始時間
    prev_frame_count: u32, // 前回のフレームカウント
    fps: f64, // 計算されたFPS
}

impl App {
    fn init() {
        let w = WIDTH;
        let h = HEIGHT;

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

        let start_time = Instant::now();
        let prev_frame_count = pyxel.frame_count;
        let fps = 0.0;

        let app = App { w, h, fishes, start_time, prev_frame_count, fps };
        pyxel.run(app);
    }
}

impl PyxelCallback for App {
    fn update(&mut self, pyxel: &mut Pyxel) {
        if pyxel.btnp(pyxel::KEY_Q, None, None) {
            pyxel.quit();
        }

        if pyxel.btnp(pyxel::MOUSE_BUTTON_LEFT, None, None) {
            let x = pyxel.mouse_x;
            let y = pyxel.mouse_y;
        }

        let fishes_clone = self.fishes.clone();
        for fish in &mut self.fishes {
            fish.update(&fishes_clone);
        }

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
        for fish in &self.fishes {
            //pyxel.rect(fish.x as f64, fish.y as f64, 4.0, 2.0, 7);
            // 進行方向を計算（速度ベクトルの角度）
            let angle = fish.vy.atan2(fish.vx);

            // 三角形の頂点を計算
            let size = 2.0; // 魚のサイズ
            let x1 = fish.x + size * angle.cos(); // 先端（頭）
            let y1 = fish.y + size * angle.sin();
            let x2 = fish.x + size * (angle + 2.0 * PI / 3.0).cos(); // 左後部
            let y2 = fish.y + size * (angle + 2.0 * PI / 3.0).sin();
            let x3 = fish.x + size * (angle - 2.0 * PI / 3.0).cos(); // 右後部
            let y3 = fish.y + size * (angle - 2.0 * PI / 3.0).sin();

            // 三角形を描画
            pyxel.tri(x1, y1, x2, y2, x3, y3, fish.color); // 白色で描画
        }

        // FPSの表示
        let fps_text = format!("FPS: {:.2}", self.fps);
        pyxel.text(5.0, 5.0, &fps_text, 7, None);
    }
}

pub fn main() {
    App::init();
}
