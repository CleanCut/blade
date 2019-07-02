use rusty_sword_arena::audio::Audio;

pub fn audio_loop() {
    let mut audio = Audio::new();
    audio.add("die", "media/die.ogg");
    audio.add("join", "media/join.ogg");
    audio.add("miss", "media/miss.ogg");
    audio.add("ow", "media/ow.ogg");
    audio.add("spawn", "media/spawn.ogg");
    audio.add("startup", "media/startup.ogg");

    audio.play("startup");

    loop {}
}
