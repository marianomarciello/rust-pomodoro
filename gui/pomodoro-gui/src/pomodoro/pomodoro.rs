use iced::widget::{button, column, text, Column, row};
use iced::{Settings, Sandbox, Element};
use crate::pomodoro::message::Message;

pub struct Pomodoro {
    // pomodoro duration in minutes
    pomodoro_duration: i32,
    // pomodoro counter
    pomodoro_counter: i32,
    // pomodoro break duration in minutes
    pomodoro_break_duration: i32,
    // pomodoro break duration counter
    pomodoro_break_counter: i32,
}

impl Sandbox for Pomodoro {
    type Message = Message;

    fn new() -> Self {
        Self {
            pomodoro_duration: 0,
            pomodoro_counter: 0,
            pomodoro_break_duration: 0,
            pomodoro_break_counter: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn view(&self) -> Element<Message> {
        // use a simple vertical layout
        row![
            // the increment button produce: IncrementPressed
            text("Pomodoro's duration:"),
            column![

                button("+").on_press(Message::IncrementPressed),

                // Show the value
                text(self.pomodoro_duration).size(50),
                // the decrement button produce: DecrementPresse
                button("-").on_press(Message::DecrementPressed),
            ],
        ]
        .padding(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.pomodoro_duration+= 1;
            }
            Message::DecrementPressed => {
                self.pomodoro_duration-= 1;
            }
        }
    }
}
