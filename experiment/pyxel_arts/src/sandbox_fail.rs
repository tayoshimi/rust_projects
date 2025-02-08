use pyxel::{Pyxel, PyxelCallback};

use std::thread;
use pyxel::{init, cls, text, draw_rectangle};

struct Board {
    data: [[i32; 3]; 2],
}

impl Board {
    fn new() -> Self {
        let mut data: [[i32; 3]; 2] = [[; 3]; 2];
        for i in 0..=2 {
            for j in 0..=1 {
                data[i][j] = 0;
            }
        }
        Board { data: data }
    }
}

fn draw_grid(x: u32, y: u32, cell_size: u32, pyxel: &mut Pyxel) {
    for i in 0..3 {
        for j in 0..3 {
            let cell_x = x + j * cell_size;
            let cell_y = y + i * cell_size;
            pyxel.rect(cell_x as f64, cell_y as f64, (cell_x + cell_size) as f64, (cell_y + cell_size) as f64, 16);
        }
    }
}

pub struct App {
    w: u64,
    h: u64,
    board: mut Board,
    player_turn: String,
}

impl App {
    fn init(&self) {
        self.w = 160;
        self.h = 120;
        self.board = Board::new();
        self.player_turn = "X";

        let mut pyxel = pyxel::init(
            160,
            120,
            Some("Hello, Pyxel in Rust!"),
            None,
            None,
            None,
            None,
            None,
        );
        pyxel.mouse(true);
        pyxel.warp_mouse(10.0, 10.0);


        let app = App { x: 0.0, y: 0.0 };
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
            if self.board.data[y][x] == 0 {
                self.board.data[y][x] = self.player_turn.to_char().into();
                self.player_turn = match self.player_turn {
                    "X" => "O",
                    "O" => "X",
                    _ => panic!("Invalid player turn"),
                };
            }
        }
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(0);
        pyxel.text(10.0, 20.0, &format!("Player {}'s turn!", self.player_turn), 10, None);
        pyxel.rect(40.0, 40.0, 120.0, 80.0, 16);

        draw_grid(40, 40, 20, pyxel);
    }
}
