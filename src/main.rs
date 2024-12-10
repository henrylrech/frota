use rodio::{OutputStream, Source};
use std::fs::File;
use std::io::BufReader;
use enigo::{Enigo, Mouse, Settings};
use enigo::Coordinate::Abs;
use std::{thread, time};

fn main() {
    //wallpaper::set_from_url("https://source.unsplash.com/random").unwrap();

    let audio_thread = thread::spawn(|| {
        play_uepa();
    });

    let mouse_thread = thread::spawn(|| {
        lock_mouse();
    });

    let image_thread = thread::spawn(|| {
        display_image();
    });

    let _ = audio_thread.join();
    let _ = mouse_thread.join();
    let _ = image_thread.join();
}

fn play_uepa() {
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to get default audio output stream");

    let file = File::open("assets/uepa.mp3").expect("Failed to open audio file");
    let source = rodio::Decoder::new(BufReader::new(file)).expect("Failed to decode audio file");

    stream_handle.play_raw(source.convert_samples()).expect("Failed to play audio");

    std::thread::sleep(time::Duration::from_secs(45)); // Adjust this duration to match the audio length
}

fn lock_mouse() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    loop {
        enigo.move_mouse(960, 540, Abs).unwrap();

        std::thread::sleep(time::Duration::from_millis(100));
    }
}

fn display_image() {
    image::open("assets/braia.jpg").expect("could not open brian");
}
