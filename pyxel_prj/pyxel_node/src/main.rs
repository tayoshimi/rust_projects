use pyxel::{Pyxel, PyxelCallback};

mod node_manager;
use node_manager::NodeManager;

mod setup_layout;


pub struct App {
    x: f64,
    y: f64,
    node_manager: NodeManager,
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

        let mut node_manager = NodeManager::new(WIDTH as f32, HEIGHT as f32);

        for i in 0..12 {
            let name = format!("node {i}");
            node_manager.add_node(&name);
        }

        node_manager.add_edge(0, 1);
        node_manager.add_edge(0, 2);
        node_manager.add_edge(1, 3);
        node_manager.add_edge(3, 4);
        node_manager.add_edge(3, 5);
        node_manager.add_edge(5, 6);
        node_manager.add_edge(2, 8);
        node_manager.add_edge(3, 9);

        node_manager.add_edge(0, 7);
        node_manager.add_edge(10, 11); // 0から辿れないので現在は描画されない

        setup_layout::setup_tree_layout(&mut node_manager);
        

        let app = App { x: 0.0, y: 0.0, node_manager: node_manager };
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
            setup_layout::setup_tree_layout(&mut self.node_manager);
        }
        if pyxel.btnp(pyxel::KEY_S, None, None) {
            setup_layout::setup_spring_layout(&mut self.node_manager);
        }
        self.node_manager.update(pyxel);
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(3);
        self.node_manager.draw(pyxel);
    }
}

pub fn main() {
    App::init();
}
