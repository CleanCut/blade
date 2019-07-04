use blade::audio;
use rusty_sword_arena::{game::GameEvent, gfx::Window};
use std::sync::mpsc;
use std::thread;

fn main() {
    // Welcome & argument parsing
    println!("Welcome to Blade, the best Rusty Sword Arena Client, ever.");
    let name = "Zork";
    let host = "localhost";
    println!("Connecting {} to server {}", name, host);

    // Network System

    // Audio System (separate thread)
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        audio::audio_loop(rx);
    });
    tx.send("startup").unwrap();

    // Everything else we need
    let mut window = Window::new(None, "Blade of Rustiness");

    // Game
    'gameloop: loop {
        for event in window.poll_game_events() {
            if let GameEvent::Quit = event {
                break 'gameloop;
            }
        }
    }

    // Cleanup
    tx.send("stop").unwrap();
    handle.join().unwrap();
}
