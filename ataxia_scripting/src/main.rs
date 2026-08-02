use std::sync::mpsc::channel;

use rhai::Engine;

fn main() {
    // rhai::CustomType;
    let (sender, recv) = channel();
    let mut x = Engine::new();
    x.on_print(move |f| {
        let _ = sender.send(f.to_string());
    });

    let r = x.run("print(40 + 2)");
    while let Ok(s) = recv.recv() {
        println!("{s}");
    }
}
