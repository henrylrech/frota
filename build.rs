extern crate winres;

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/my_icon.ico");
    res.compile().unwrap();
  }
}
