use pyxel::{Pyxel, PyxelCallback};


pub struct Node {
    //name: String,
    x: f64,
    y: f64,
    color: u8,
    //pub depth: usize,
}

impl Node {
    pub const NORMAL_W:f64 = 12.0;
    pub const NORMAL_H:f64 = 10.0;
    pub const FONT_W:f64 = 4.0;
    pub const FONT_H:f64 = 8.0;

    pub fn new(x: f64,
               y: f64) -> Node {
                   Node {
                       x: x, y: y, color: pyxel::COLOR_RED
                   }

               }

               pub fn update(&mut self, pyxel: &mut Pyxel) {
               }

               pub fn set_pos(&mut self, x: f64, y: f64) {
                   self.x = x;
                   self.y = y;
               }

               pub fn move_pos(&mut self, vx: f64, vy: f64) {
                   self.x = self.x + vx;
                   self.y = self.y + vy;
               }

               pub fn get_center(&self) -> (f64, f64) {
                   (self.x + Node::NORMAL_W / 2.0, self.y + Node::NORMAL_H / 2.0)
               }

               /*fn get_text_draw_pos(&mut self) -> (f64, f64) {
                   let tx: f64 = self.x + (Node::NORMAL_W - Node::FONT_W * self.name.len() as f64) / 2.0;
                   let ty: f64 = self.y + (Node::NORMAL_H - Node::FONT_H) / 2.0;
                   (tx, ty)
               }*/

               pub fn draw(&mut self, pyxel: &mut Pyxel) {
                   pyxel.elli(self.x, self.y, Node::NORMAL_W, Node::NORMAL_H, self.color);
                   pyxel.ellib(self.x, self.y, Node::NORMAL_W, Node::NORMAL_H, pyxel::COLOR_WHITE);
                   //let (tx, ty) = self.get_text_draw_pos();
                   //pyxel.text(tx, ty, &self.name, 10, None);
                   //pyxel.text(tx, ty, &self.depth.to_string(), 10, None);

               }

}

pub struct NodeManager {
    pub pool: Vec<Node>,
    pub world_w: f64,
    pub world_h: f64,
    count: i64,
}

impl NodeManager {
    pub fn new(
        world_w: f64,
        world_h: f64) -> NodeManager {
            let pool = Vec::new();
            NodeManager {
                pool: pool,
                world_w: world_w,
                world_h: world_h,
                count: 0,
            }
        }

        pub fn add_node(&mut self, x: i32, y: i32) {
            self.pool.push(Node::new(x as f64, y as f64));
        }

        pub fn update(&mut self, pyxel: &mut Pyxel) {
        }

        pub fn draw(&mut self, pyxel: &mut Pyxel) {
            for mut p in &mut self.pool {
                p.draw(pyxel);
            }
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


        let mut nodeMan = NodeManager::new(w as f64, h as f64);
        nodeMan.add_node(50, 40);

        let app = App { w: w, h: h, node_man: nodeMan };
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

            self.node_man.add_node(x, y);
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
