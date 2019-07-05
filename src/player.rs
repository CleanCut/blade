use rusty_sword_arena::{
    game::{PlayerEvent, PlayerState},
    gfx::{Img, Window},
    timer::Timer,
};
use std::f32::consts::PI;
use std::sync::mpsc::Sender;
use std::time::Duration;

pub struct Player {
    audio_tx: Sender<&'static str>,
    pub player_state: PlayerState,
    player_img: Img,
    sword_img: Img,
    rip_img: Img,
    sword_timer: Timer,
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
        let sword_img = Img::new(
            window,
            player_state.pos,
            player_state.direction,
            None,
            "media/sword.png",
        );
        let rip_img = Img::new(
            window,
            player_state.pos,
            0.0,
            Some(player_state.color),
            "media/rip.png",
        );
        let mut sword_timer = Timer::from_millis(350);
        sword_timer.update(Duration::from_millis(351));
        Self {
            audio_tx,
            player_state,
            player_img,
            sword_img,
            rip_img,
            sword_timer,
        }
    }
    pub fn update_state(&mut self, player_state: PlayerState) {
        self.player_state = player_state;
        let ps = &mut self.player_state;
        self.player_img.pos = ps.pos;
        self.player_img.direction = ps.direction;
        self.sword_img.pos = ps.pos;
        self.rip_img.pos = ps.pos;
        for player_event in ps.player_events.drain(..) {
            // Reset the sword timer when we attack
            match player_event {
                PlayerEvent::AttackHit { .. } => self.sword_timer.reset(),
                PlayerEvent::AttackMiss => self.sword_timer.reset(),
                _ => {}
            }
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
        // The timer being "ready" means the sword swing is over, so just point the sword forward
        if self.sword_timer.ready {
            self.sword_img.direction = ps.direction;
        } else {
            // If the timer is going, then put the sword in some portion of the swing animation
            self.sword_img.direction =
                ps.direction + (2.0 * PI * self.sword_timer.time_left_percent());
        }
    }
    pub fn draw(&self, window: &mut Window) {
        if self.player_state.dead {
            if !self.player_state.joining {
                window.draw(&self.rip_img);
            }
            return;
        }
        window.draw(&self.player_img);
        window.draw(&self.sword_img);
    }
    pub fn update_timer(&mut self, dt: Duration) {
        self.sword_timer.update(dt);
    }
}