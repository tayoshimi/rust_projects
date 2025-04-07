use pyxel::{Pyxel, PyxelCallback};
use std::iter;

#[derive(Clone)]
struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    color: u8,
    speed: f64,
    deg: f64,
    timer: u16,
    alive: bool,
}

impl Particle {
    fn new() -> Self {
        Particle {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            color: 0,
            speed: 0.0,
            deg: 0.0,
            timer: 0,
            alive: false,
        }
    }

    fn update(&mut self, pyxel: &mut Pyxel) {
        self.vx = self.vx * 0.97;
        self.vy = self.vy * 0.97;
        self.timer += 1;
        if self.timer > 60 {
            self.alive = false;
        }
        self.x = self.x + self.vx;
        self.y = self.y + self.vy;
    }

    fn draw(&self, pyxel: &mut Pyxel) {
        //screen.set_color();
        //pyxel.pset(self.x as f64, self.y as f64, self.vx * 2.0, self.vy * 2.0);
        pyxel.pset(self.x, self.y, self.color);
    }

    // def setSpeed(&self, deg: int, speed: float) {
    //     self.deg, self.speed = deg, speed
    //     self.vx, self.vy = speed * pyxel.cos(deg), speed * -pyxel.sin(deg);
    // }
}

struct Firework {
    particles: Vec<Particle>,
}

impl Firework {
    fn new(pool_size: usize) -> Self {
        let init_p = Particle::new(); 
        let particles: Vec<Particle> = iter::repeat(init_p)
            .take(pool_size)
            .map(|v| v.clone())
            .collect();
        Firework { particles: particles }
    }

    fn add_particle(&mut self, x: i32, y: i32, deg: f64, speed: f64, color: u8) {
        let mut particle = Particle::new();
 
        //self.particles.push(particle);
        if let Some(particle) = self.particles.iter_mut().find(|particle| particle.alive == false) {
            particle.x = x as f64;
            particle.y = y as f64;
            particle.color = color;
            particle.speed = speed;
            particle.deg = deg;
            particle.vx = speed * Pyxel::cos(deg);
            particle.vy = speed * -Pyxel::sin(deg);
            particle.timer = 0;
            particle.alive = true;
        } else {
            eprintln!("err");
        }
    }

    fn add_fires(&mut self, x: i32, y: i32) {
        for i in 0..32 {
            let deg = Pyxel::rndi(0, 360) as f64;
            let speed = 0.1 + Pyxel::rndf(0.1, 1.0) * 1.5;
            let color =Pyxel::rndi(8,10) as u8;
            self.add_particle(x, y, deg, speed, color);
        }
    }

    fn update(&mut self, pyxel: &mut Pyxel) {
        self.particles.iter_mut().filter(|p| p.alive).for_each(|particle| {
            particle.update(pyxel);
        });
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        self.particles.iter_mut().filter(|p| p.alive).for_each(|particle| {
            particle.draw(pyxel);
        });
    }
}

pub struct App {
    w: u32,
    h: u32,
    firework: Firework,
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


        let mut firework = Firework::new(200);

        let app = App { w: w, h: h, firework: firework };
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

            self.firework.add_fires(x, y);
        }

        self.firework.update(pyxel);
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(1);
        //pyxel.text(10.0, 20.0, &format!("Player {}'s turn!", self.player_turn), 10, None);

        self.firework.draw(pyxel);
    }
}

pub fn main() {
    App::init();
}
