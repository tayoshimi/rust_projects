use pyxel::{Pyxel, PyxelCallback};

mod node_manager;
use node_manager::{Node, NodeManager};

mod setup_layout;
use setup_layout::SetupTreeLayout;


pub struct App {
    x: f64,
    y: f64,
    nodeManager: NodeManager,
}

impl App {
    fn init() {
        let mut pyxel = pyxel::init(
            400,
            350,
            Some("Hello, Pyxel in Rust!"),
            None,
            None,
            None,
            None,
            None,
        );
        pyxel.mouse(true);
        pyxel.warp_mouse(10.0, 10.0);

        let mut nodeManager = NodeManager::new(400.0, 350.0);

        for i in 0..10 {
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
        //nodeManager.add_edge(10, 11);

        setup_layout::SetupTreeLayout(&mut nodeManager);

        let app = App { x: 0.0, y: 0.0, nodeManager: nodeManager };
        pyxel.run(app);
    }

    //fn update(&mut self, pyxel: &mut Pyxel);
    //fn draw(&mut self, pyxel: &mut Pyxel);
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
