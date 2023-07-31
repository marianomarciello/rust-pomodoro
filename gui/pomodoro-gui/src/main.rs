mod pomodoro;
use pomodoro::Pomodoro;
use iced::{Application, Settings};


fn main() {
    Pomodoro::run(Settings::default());
    println!("Hello, world!");
}
