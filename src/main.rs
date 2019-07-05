use blade::{audio, player::Player};
use rusty_sword_arena::game::ButtonProcessor;
use rusty_sword_arena::{
    game::{GameEvent, PlayerInput, Vector2},
    gfx::Window,
    net::ConnectionToServer,
    VERSION,
};
use std::{collections::HashMap, env, process, sync::mpsc, thread};

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
    let mut players: HashMap<u8, Player> = HashMap::new();
    let mut mouse_pos = Vector2::new();
    let mut player_input = PlayerInput::with_id(my_id);
    let mut button_processor = ButtonProcessor::new();

    // Game
    'gameloop: loop {
        // Accumulate & send player input
        for event in window.poll_game_events() {
            match event {
                GameEvent::Quit => break 'gameloop,
                GameEvent::MouseMoved { position } => mouse_pos = position,
                GameEvent::Button {
                    button_state,
                    button_value,
                } => button_processor.process(button_state, button_value, &mut player_input),
            }
        }
        if let Some(my_player) = players.get(&my_id) {
            // If I know my position, I can set my direction to point towards the mouse
            player_input.direction = my_player.player_state.pos.angle_between(mouse_pos);
        }
        connection.send_player_input(&player_input);

        // Process any new game states
        for game_state in connection.poll_game_states() {
            // Remove players who no longer have a game state
            players.retain(|k, _| game_state.player_states.contains_key(k));
            // Create new players and update existing players
            for (id, player_state) in game_state.player_states {
                players
                    .entry(id)
                    .or_insert_with(|| Player::new(&window, tx.clone(), player_state.clone()))
                    .update_state(player_state);
            }
        }

        // Update player timers

        // Draw a frame
        window.drawstart();
        for (id, player) in &players {
            if *id == my_id {
                continue
            }
            player.draw(&mut window);
        }
        if let Some(player) = players.get(&my_id) {
            player.draw(&mut window);
        }
        window.drawfinish();

        // Do timekeeping
    }

    // Cleanup
    tx.send("stop").unwrap();
    handle.join().unwrap();
}
