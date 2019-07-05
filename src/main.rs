use blade::audio;
use rusty_sword_arena::{game::GameEvent, gfx::Window, net::ConnectionToServer, VERSION};
use std::{env, process, sync::mpsc, thread};

fn main() {
    // Welcome & argument parsing
    println!("Welcome to Blade, the best Rusty Sword Arena Client, ever.");
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Usage: (prog) name host");
        process::exit(2);
    }
    let host = args.pop().unwrap();
    let name = args.pop().unwrap();
    println!("Connecting {} to server {}", name, host);

    // Network System
    let mut connection = ConnectionToServer::new(&host);
    let settings = connection.get_game_settings();
    println!(
        "Client v{} connected to server v{} at {}",
        VERSION, settings.version, host
    );
    let my_id = match connection.join(&name) {
        Ok(id) => id,
        Err(message) => {
            println!("{}", message);
            process::exit(3);
        }
    };
    println!("My player id is {}", my_id);

    // Audio System (separate thread)
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        audio::audio_loop(rx);
    });
    tx.send("startup")
        .expect("Audio thread seems to have crashed");

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
