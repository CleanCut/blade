use rusty_sword_arena::audio::Audio;
use std::sync::mpsc::Receiver;

pub fn audio_loop(rx: Receiver<&str>) {
    let mut audio = Audio::new();
    audio.add("die", "media/die.ogg");
    audio.add("join", "media/join.ogg");
    audio.add("miss", "media/miss.ogg");
    audio.add("ow", "media/ow.ogg");
    audio.add("spawn", "media/spawn.ogg");
    audio.add("startup", "media/startup.ogg");

    loop {
        let clip = rx.recv().unwrap();
        audio.play(clip);
    }
}
