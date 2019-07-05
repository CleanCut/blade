use rusty_sword_arena::{
    game::{PlayerEvent, PlayerState},
    gfx::{Img, Window},
};
use std::sync::mpsc::Sender;

pub struct Player {
    audio_tx: Sender<&'static str>,
    pub player_state: PlayerState,
    player_img: Img,
}

impl Player {
    pub fn new(window: &Window, audio_tx: Sender<&'static str>, player_state: PlayerState) -> Self {
        let player_img = Img::new(
            window,
            player_state.pos,
            player_state.direction,
            Some(player_state.color),
            "media/player.png",
        );
        Self {
            audio_tx,
            player_state,
            player_img,
        }
    }
    pub fn update_state(&mut self, player_state: PlayerState) {
        self.player_state = player_state;
        let ps = &mut self.player_state;
        self.player_img.pos = ps.pos;
        self.player_img.direction = ps.direction;
        for player_event in ps.player_events.drain(..) {
            // Play sounds
            match player_event {
                PlayerEvent::AttackMiss => self.audio_tx.send("miss").unwrap(),
                PlayerEvent::Die => self.audio_tx.send("die").unwrap(),
                PlayerEvent::Join => self.audio_tx.send("join").unwrap(),
                PlayerEvent::Spawn => self.audio_tx.send("spawn").unwrap(),
                PlayerEvent::TookDamage => self.audio_tx.send("ow").unwrap(),
                _ => {}
            }
        }
    }
    pub fn draw(&self, window: &mut Window) {
        if self.player_state.dead {
            return;
        }
        window.draw(&self.player_img);
    }
}
