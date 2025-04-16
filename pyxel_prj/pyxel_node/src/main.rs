use pyxel::{Pyxel, PyxelCallback};

mod node_manager;
use node_manager::{Node, NodeManager};

mod setup_layout;
use setup_layout::{setup_tree_layout, setup_spring_layout};


pub struct App {
    x: f64,
    y: f64,
    nodeManager: NodeManager,
}

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

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

        let mut nodeManager = NodeManager::new(WIDTH as f64, HEIGHT as f64);

        for i in 0..12 {
            let name = format!("node {i}");
            nodeManager.add_node(&name);
        }

        nodeManager.add_edge(0, 1);
        nodeManager.add_edge(0, 2);
        nodeManager.add_edge(1, 3);
        nodeManager.add_edge(3, 4);
        nodeManager.add_edge(3, 5);
        nodeManager.add_edge(5, 6);
        nodeManager.add_edge(2, 8);
        nodeManager.add_edge(3, 9);

        nodeManager.add_edge(0, 7);
        nodeManager.add_edge(10, 11); // 0から辿れないので現在は描画されない

        setup_layout::setup_tree_layout(&mut nodeManager);
        

        let app = App { x: 0.0, y: 0.0, nodeManager: nodeManager };
        pyxel.run(app);
    }
}

impl PyxelCallback for App {
    fn update(&mut self, pyxel: &mut Pyxel) {
        if pyxel.frame_count < 60 {
            self.x += (pyxel.frame_count % 2) as f64;
            self.y -= 1.0;
        }

        if pyxel.btnp(pyxel::KEY_Q, None, None) {
            pyxel.quit();
        }
        if pyxel.btnp(pyxel::KEY_T, None, None) {
            setup_layout::setup_tree_layout(&mut self.nodeManager);
        }
        if pyxel.btnp(pyxel::KEY_S, None, None) {
            setup_layout::setup_spring_layout(&mut self.nodeManager);
        }
        self.nodeManager.update(pyxel);
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(3);
        self.nodeManager.draw(pyxel);
    }
}

pub fn main() {
    App::init();
}
