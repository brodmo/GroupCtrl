use crate::app::App;
use crate::open::Open;

mod app;
mod open;

fn main() {
    App::new("com.apple.finder").open().unwrap();
}
