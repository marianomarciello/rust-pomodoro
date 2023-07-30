mod pomodoro;
use pomodoro::Pomodoro;
use iced::{Sandbox, Settings};


fn main() {
    Pomodoro::run(Settings::default());
    println!("Hello, world!");
}
