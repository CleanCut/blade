use blade::audio;
use rusty_sword_arena::{game::GameEvent, gfx::Window};
use std::sync::mpsc;
use std::thread;

fn main() {
    println!("Welcome to Blade, the best Rusty Sword Arena Client, ever.");
    let name = "Zork";
    let host = "localhost";
    println!("Connecting {} to server {}", name, host);

    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        audio::audio_loop(rx);
    });
    tx.send("startup").unwrap();

    let mut window = Window::new(None, "Blade of Rustiness");
    'gameloop: loop {
        for event in window.poll_game_events() {
            if let GameEvent::Quit = event {
                break 'gameloop;
            }
        }
    }

    tx.send("stop").unwrap();
    handle.join().unwrap();
}
