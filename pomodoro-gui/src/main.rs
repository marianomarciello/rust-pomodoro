mod gui;
use iced::{Application, Settings};
use gui::Pomodoro;

fn main() {
    let _ = Pomodoro::run(Settings::default());
    println!("Hello, world!");
}
