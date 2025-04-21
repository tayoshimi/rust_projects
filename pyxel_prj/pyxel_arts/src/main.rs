use pyxel::{Pyxel, PyxelCallback};
mod firework;
//use firework;

mod many_rect;
//use many_rect;

pub fn main() {
    let mut pyxel = pyxel::init(
        200,
        200,
        Some("Hello, Pyxel in Rust!"),
        None,
        None,
        None,
        None,
        None,
    );
    pyxel.mouse(true);
    pyxel.warp_mouse(10.0, 10.0);

    let app_name = "App1";

    // 選択された App をトレイトオブジェクトとして生成
    let mut app: Box<dyn PyxelCallback> = match app_name {
        "App1" => Box::new(firework::App::init(&mut pyxel)),
        "App2" => Box::new(many_rect::App::init(&mut pyxel)),
        _ => {
            eprintln!("未知のアプリ: {}. 'App1' を使用します。", app_name);
            Box::new(firework::App::init(&mut pyxel))
        }
    };

    pyxel.run(&mut app);
}
