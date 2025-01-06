extern crate winres;
use std::fs;

fn main() {
    if cfg!(target_os = "windows") {
        let icon_path = "assets/icon.ico";
        
        if fs::metadata(icon_path).is_ok() {
            let mut res = winres::WindowsResource::new();
            res.set_icon(icon_path);
            res.compile().unwrap();
        }
    }
}
