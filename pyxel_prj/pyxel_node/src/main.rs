use pyxel::{Pyxel, PyxelCallback};

mod node_manager;
use node_manager::NodeManager;

mod setup_layout;


pub struct App {
    // x: f64,
    // y: f64,
    camera_x: f32,
    camera_y: f32,
    dragging: bool,
    last_mouse_x: i32,
    last_mouse_y: i32,
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
        

        let app = App {             
            camera_x: 0.0,
            camera_y: 0.0,
            dragging: false,
            last_mouse_x: 0,
            last_mouse_y: 0,
            node_manager: node_manager 
        };
        pyxel.run(app);
    }
}

impl PyxelCallback for App {
    fn update(&mut self, pyxel: &mut Pyxel) {
        if pyxel.frame_count < 60 {
            //self.x += (pyxel.frame_count % 2) as f64;
            //self.y -= 1.0;
        }

                // ドラッグ開始（左ボタンを押した瞬間）
        if pyxel.btnp(pyxel::MOUSE_BUTTON_LEFT, None, None) {
            self.dragging = true;
            self.last_mouse_x = pyxel.mouse_x;
            self.last_mouse_y = pyxel.mouse_y;
        }

        // ドラッグ中（押し続け）
        if self.dragging && pyxel.btn(pyxel::MOUSE_BUTTON_LEFT) {
            let mx = pyxel.mouse_x;
            let my = pyxel.mouse_y;
            let dx = mx - self.last_mouse_x;
            let dy = my - self.last_mouse_y;
            self.camera_x += dx as f32;
            self.camera_y += dy as f32;
            self.last_mouse_x = mx;
            self.last_mouse_y = my;
        }

        // ドラッグ終了（ボタンが離れたらフラグを下ろす）
        if self.dragging && !pyxel.btn(pyxel::MOUSE_BUTTON_LEFT) {
            self.dragging = false;
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
        self.node_manager.draw(pyxel, self.camera_x as f32, self.camera_y as f32);
    }
}

pub fn main() {
    App::init();
}
