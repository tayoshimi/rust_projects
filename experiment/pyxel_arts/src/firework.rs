use pyxel::{Pyxel, PyxelCallback};
use std::iter;

#[derive(Clone)]
struct Particle {
    x: i32,
    y: i32,
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
            x: 0,
            y: 0,
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
        self.x = self.x + self.vx as i32;
        self.y = self.y + self.vy as i32;
    }

    fn draw(&self, pyxel: &mut Pyxel) {
        //screen.set_color();
        //pyxel.pset(self.x as f64, self.y as f64, self.vx * 2.0, self.vy * 2.0);
        pyxel.pset(self.x as f64, self.y as f64, self.color);
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

    fn add_particle(&mut self, x: i32, y: i32, deg: f64, speed: f64, color: u8, pyxel: &mut Pyxel) {
        let mut particle = Particle::new();
 
        //self.particles.push(particle);
        if let Some(particle) = self.particles.iter_mut().find(|particle| particle.alive == false) {
            particle.x = x;
            particle.y = y;
            particle.color = color;
            particle.speed = speed;
            particle.deg = deg;
            particle.vx = speed * pyxel.cos(deg);
            particle.vy = speed * -pyxel.sin(deg);
            particle.timer = 0;
            particle.alive = true;
        } else {
            eprintln!("err");
        }
    }

    fn add_fires(&mut self, x: i32, y: i32, pyxel: &mut Pyxel) {
        for i in 0..32 {
            let deg = pyxel.rndf(0.0, 360.0);
            let speed = 0.1 + pyxel.rndf(0.0, 1.0) * 1.5;
            let color =pyxel.rndi(8,10) as u8;
            self.add_particle(x, y, deg, speed, color, pyxel);
        }
    }

    fn update(&mut self, pyxel: &mut Pyxel) {
        self.particles.iter_mut().filter(|p| p.alive).for_each(|particle| {
            particle.update(pyxel);
        });
    }

    // fn cleanup(&mut self) {
    //     let mut new_particles: Vec<Particle> = Vec::new();
    //     for particle in &self.particles {
    //         if let Some(particle) = particle {
    //             if particle.alive {
    //                 new_particles.push(*particle);
    //             }
    //         }
    //     }
    //     self.particles = new_particles;
    // }

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


        let mut firework = Firework::new(100);

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

            self.firework.add_fires(x, y, pyxel);
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


// struct ParticleAppTest {
//     manager: GameObjectManager,
// }

// impl ParticleAppTest {
//     pub fn new() -> Self {
//         ParticleAppTest {
//             manager: GameObjectManager::new(256, &Particle::new()),
//         }
//     }

//     fn run(&mut self) {
//         if pyxel::button_check(pyxel::KEY_Q) {
//             pyxel::quit();
//         }

//         if pyxel::mouse_check_left() {
//             for i in 0..32 {
//                 let deg = rand::random::<f32>() * std::f32::consts::PI;
//                 let speed = 0.1 + rand::random::<f32>() * 1.5;
//                 let color = match pyxel::COLOR.White {
//                     _ => pyxel::COLOR_Red,
//                 };
//                 ParticleAppTest::_add_particle(&mut self._manager, pyxel::mouse_x as u16, pyxel::mouse_y as u16, deg, speed, color);
//             }
//         }

//         self._manager.update();
//         self._manager.cleanup();

//         draw();
//     }

//     fn draw(&mut self) {
//         pyxel::cls(0);
//         self._manager.draw();
//         pyxel::text(4, pyxel::height - 16, format!("particle: {}", self._manager.particles.len()), pyxel::COLOR_White);
//     }

//     fn _add_particle(&mut self, x: u16, y: u16, deg: f32, speed: f32, color: u16) {
//         let mut particle = Particle::new();
//         particle.x = x;
//         particle.y = y;
//         particle.vx = speed * Pyxel:PI / 180.0 * deg;
//         particle.vy = 0.0;
//         particle.speed = speed;
//         particle.timer = 0;
//         self._manager.add_particle(x, y, deg, speed, color);
//     }
// }


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
