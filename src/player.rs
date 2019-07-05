use rusty_sword_arena::{
    game::PlayerState,
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
    }
    pub fn draw(&self, window: &mut Window) {
        if self.player_state.dead {
            return;
        }
        window.draw(&self.player_img);
    }
}
