use pyxel::{Pyxel, PyxelCallback};

struct Particle {
    x: u16,
    y: u16,
    vx: f64,
    vy: f64,
    speed: f32,
    timer: u16,
    alive: bool,
}

impl Particle {
    fn new() -> Self {
        Particle {
            x: 0,
            y: 0,
            vx: 0.0,
            vy: 0.0,
            speed: 0.0,
            timer: 0,
            alive: false,
        }
    }

    fn update(&mut self, pyxel: &mut Pyxel) {
        self.vx *= 0.97;
        self.vy *= 0.97;
        self.timer += 1;
        if self.timer > 60 {
            self.alive = false;
        }
    }

    fn draw(&self, pyxel: &mut Pyxel) {
        //screen.set_color();
        //pyxel.pset(self.x as f64, self.y as f64, self.vx * 2.0, self.vy * 2.0);
        pyxel.pset(self.x as f64, self.y as f64, pyxel::WHITE);
    }
}

struct Firework {
    particles: Vec<Particle>,
}

impl Firework {
    fn new(size: u16) -> Self {
        Firework { particles: Vec::new() }
    }

    fn add_particle(&mut self, x: u16, y: u16, deg: f32, speed: f32, color: u16) {
        let mut particle = Particle::new();
        particle.x = x;
        particle.y = y;
        particle.vx = Pyxel::PI / 180.0 * deg;
        particle.vy = 0.0;
        particle.speed = speed;
        particle.timer = 0;
        particle.alive = true;
        self.particles.push(particle);
    }

    fn update(&mut self, pyxel: &mut Pyxel) {
        let mut active_articles = &self.particles.filter(|p| p.alive);
        for particle in active_articles {
            if let Some(particle) = particle {
                particle.vx *= 0.97;
                particle.vy *= 0.97;
                particle.timer += 1;
                if particle.timer > 60 {
                    particle.alive = false;
                }
            }
        }
    }

    fn cleanup(&mut self) {
        let mut new_particles: Vec<Particle> = Vec::new();
        for particle in &self.particles {
            if let Some(particle) = particle {
                if particle.alive {
                    new_particles.push(*particle);
                }
            }
        }
        self.particles = new_particles;
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        for particle in &self.particles {
            if let Some(particle) = particle {
                screen.set_color(pyxel::COLOR_WHITE);
                screen.plot_particle(particle.x as i32, particle.y as i32, particle.vx * 2.0, particle.vy * 2.0);
            }
        }
    }
}

struct ParticleAppTest {
    manager: GameObjectManager,
}

impl ParticleAppTest {
    pub fn new() -> Self {
        ParticleAppTest {
            manager: GameObjectManager::new(256, &Particle::new()),
        }
    }

    fn run(&mut self) {
        if pyxel::button_check(pyxel::KEY_Q) {
            pyxel::quit();
        }

        if pyxel::mouse_check_left() {
            for i in 0..32 {
                let deg = rand::random::<f32>() * std::f32::consts::PI;
                let speed = 0.1 + rand::random::<f32>() * 1.5;
                let color = match pyxel::COLOR.White {
                    _ => pyxel::COLOR_Red,
                };
                ParticleAppTest::_add_particle(&mut self._manager, pyxel::mouse_x as u16, pyxel::mouse_y as u16, deg, speed, color);
            }
        }

        self._manager.update();
        self._manager.cleanup();

        draw();
    }

    fn draw(&mut self) {
        pyxel::cls(0);
        self._manager.draw();
        pyxel::text(4, pyxel::height - 16, format!("particle: {}", self._manager.particles.len()), pyxel::COLOR_White);
    }

    fn _add_particle(&mut self, x: u16, y: u16, deg: f32, speed: f32, color: u16) {
        let mut particle = Particle::new();
        particle.x = x;
        particle.y = y;
        particle.vx = speed * Pyxel:PI / 180.0 * deg;
        particle.vy = 0.0;
        particle.speed = speed;
        particle.timer = 0;
        self._manager.add_particle(x, y, deg, speed, color);
    }
}


/*
struct Explosion {
    fn init() {
        ParticleManager = GameObjectManager(256, Particle::new());
    }
}

impl Explosion {
    fn explode(&self, x: u16, y: u16) {
        for _ in 0..32 {
            let deg = rand::random::<f64>() * 360.0;
            let speed = 0.1 + rand::random::<f64>() * 1.5;
            let color = match random::random::<u8>() % 3 {
                0 => pyxel::COLOR_WHITE,
                1 => pyxel::COLOR_RED,
                2 => pyxel::COLOR_YELLOW,
                _ => panic!("Invalid random value"),
            };
            ParticleManager.add(x, y, deg, speed as f32, color);
        }
    }

    fn update(&self) {
        ParticleManager.update();
    }

    fn cleanup(&self) {
        ParticleManager.cleanup();
    }

    fn draw(&self) {
        ParticleManager.draw();
    }

    fn num(&self) -> u16 {
        ParticleManager.list.len() as u16
    }
}*/

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
