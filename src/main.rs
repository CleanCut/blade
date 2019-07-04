use blade::audio;
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

    handle.join().unwrap();
}
