// https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
// https://www.programiz.com/rust/module
// https://rust-classes.com/chapter_4_3.html#:~:text=Each%20Rust%20file%20is%20called,lib.rs%20for%20a%20library



mod consts;
mod lead;
mod unit;
mod food;
mod game;
mod worm;

use egui::ViewportBuilder;
use game::Game;



fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native("Worm", options, Box::new(|cc| Box::new(Game::new(cc))))
}
