use rodio::{OutputStream, Source};
use std::fs::File;
use std::io::BufReader;
use enigo::{Enigo, Mouse, Settings, Key};
use enigo::Coordinate::Abs;
use enigo::{Direction::{Click, Press, Release}, Keyboard};
use std::{thread, time};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winuser::{
    DestroyCursor, LoadCursorFromFileW, SetSystemCursor,
};

struct Config {
    mp3_path: String,
    ani_path: String
}

impl Config {
    fn new(mp3_path: &str, ani_path: &str) -> Self {
        Config {
            ani_path: String::from(ani_path),
            mp3_path: String::from(mp3_path)
        }
    }
}

fn main() {
    let config = Config::new("assets/audio.mp3", "assets/cursor.ani");
    
    wallpaper::set_from_url("https://i.redd.it/c7iaou1ua1b21.jpg").expect("could not change wallpaper"); // terry crews

    let audio_thread = thread::spawn(move || {
        play_uepa(&config.mp3_path);
    });

    let mouse_thread = thread::spawn(|| {
        lock_mouse();
    });

    go_to_desktop();
    change_cursor(&config.ani_path);
    let _ = audio_thread.join();
    let _ = mouse_thread.join();
}

fn play_uepa(mp3_path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to get default audio output stream");

    let file = File::open(mp3_path).expect("Failed to open audio file");
    let source = rodio::Decoder::new(BufReader::new(file)).expect("Failed to decode audio file");

    stream_handle.play_raw(source.convert_samples()).expect("Failed to play audio");

    std::thread::sleep(time::Duration::from_secs(45));
}

fn lock_mouse() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    loop {
        enigo.move_mouse(960, 540, Abs).unwrap();

        std::thread::sleep(time::Duration::from_millis(10000));
    }
}

fn go_to_desktop() {
    if cfg!(target_os = "windows") {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        let _ = enigo.key(Key::Meta, Press);
        let _ = enigo.key(Key::Unicode('d'), Click);
        let _ = enigo.key(Key::Meta, Release);
    }
}

fn change_cursor(ani_path: &str) {
    if cfg!(target_os = "windows") {
        let wide_path: Vec<u16> = OsStr::new(ani_path).encode_wide().chain(Some(0)).collect();

        unsafe {
            let h_cursor = LoadCursorFromFileW(wide_path.as_ptr());
            if h_cursor.is_null() {
                return;
            }

            if SetSystemCursor(h_cursor, 32512) == 0 {
                return;
            }

            DestroyCursor(h_cursor);
        }
    }
}
