mod pomodoro;
use iced::{Application, Settings};
use pomodoro::Pomodoro;

fn main() {
    Pomodoro::run(Settings::default());
    println!("Hello, world!");
}
