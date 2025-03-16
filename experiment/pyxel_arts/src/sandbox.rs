use pyxel::{Pyxel, PyxelCallback};
use std::iter;

#[derive(Clone)]
pub struct Node {
    //name: String,
    x: i32,
    y: i32,
    color: u8,
    alive: bool,
    //pub depth: usize,
}

impl Node {
    pub const NORMAL_W:f64 = 12.0;
    pub const NORMAL_H:f64 = 10.0;
    pub const FONT_W:f64 = 4.0;
    pub const FONT_H:f64 = 8.0;

    pub fn new(x: i32,
               y: i32) -> Node {
                   Node {
                       x: x, y: y, color: pyxel::COLOR_RED,
                       alive: false,
                   }

               }

               pub fn update(&mut self, pyxel: &mut Pyxel) {
               }

               pub fn set_pos(&mut self, x: i32, y: i32) {
                   self.x = x;
                   self.y = y;
               }

               pub fn move_pos(&mut self, vx: i32, vy: i32) {
                   self.x = self.x + vx;
                   self.y = self.y + vy;
               }

               pub fn get_center(&self) -> (i32, i32) {
                   (self.x + (Node::NORMAL_W / 2.0) as i32, self.y + (Node::NORMAL_H / 2.0) as i32)
               }

               /*fn get_text_draw_pos(&mut self) -> (f64, f64) {
                   let tx: f64 = self.x + (Node::NORMAL_W - Node::FONT_W * self.name.len() as f64) / 2.0;
                   let ty: f64 = self.y + (Node::NORMAL_H - Node::FONT_H) / 2.0;
                   (tx, ty)
               }*/

               pub fn draw(&mut self, pyxel: &mut Pyxel) {
                   pyxel.elli(self.x as f64, self.y as f64, Node::NORMAL_W, Node::NORMAL_H, self.color);
                   pyxel.ellib(self.x as f64, self.y as f64, Node::NORMAL_W, Node::NORMAL_H, pyxel::COLOR_WHITE);
                   //let (tx, ty) = self.get_text_draw_pos();
                   //pyxel.text(tx, ty, &self.name, 10, None);
                   //pyxel.text(tx, ty, &self.depth.to_string(), 10, None);

               }

}

pub struct NodeManager {
    pool: Vec<Node>,
    pub world_w: i32,
    pub world_h: i32,
}

impl NodeManager {
    pub fn new(
        world_w: i32,
        world_h: i32,
        pool_size: usize) -> NodeManager {
            let initial_node = Node::new(0, 0); 
            let vec: Vec<Node> = iter::repeat(initial_node)
                .take(pool_size)
                .map(|v| v.clone())
                .collect();

            NodeManager {
                pool: vec,
                world_w: world_w,
                world_h: world_h,
            }
        }

        pub fn rise_node(&mut self, x: i32, y: i32) {
            if let Some(node) = self.pool.iter_mut().find(|node| node.alive == false) {
                node.set_pos(x, y);
                node.alive = true;
                println!("pool:{}", self.pool.len());
            } else {
                eprintln!("err");
            }
        }

        pub fn update(&mut self, pyxel: &mut Pyxel) {
        }

        pub fn draw(&mut self, pyxel: &mut Pyxel) {
            // for mut p in &mut self.pool {
            //     p.draw(pyxel);
            // }
        }

}




pub struct App {
    w: u32,
    h: u32,
    node_man: NodeManager,
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


        let mut node_man = NodeManager::new(w as i32, h as i32, 100);
        node_man.rise_node(50, 40);

        let app = App { w: w, h: h, node_man: node_man };
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

            self.node_man.rise_node(x, y);
        }
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(1);
        //pyxel.text(10.0, 20.0, &format!("Player {}'s turn!", self.player_turn), 10, None);

        self.node_man.draw(pyxel);
    }
}

pub fn main() {
    App::init();
}
