use iced::widget::{button, column, text, row};
use iced::{Application, Command, Element, Subscription};
use iced::theme::{self, Theme};
use iced::executor;
use crate::pomodoro::message::Message;
use std::{thread, time};
use std::time::{Duration, Instant};

// https://github.com/iced-rs/iced/blob/0.10/examples/stopwatch/src/main.rs
struct Stopwatch {
    duration: Duration,
    state: State,
}

enum State {
    Idle,
    Ticking { last_tick: Instant },
}

pub struct Pomodoro {
    // pomodoro duration in minutes
    pomodoro_duration: u32,
    // pomodoro counter
    pomodoro_counter: u32,
    // pomodoro break duration in minutes
    break_duration: u32,
    // pomodoro break duration counter
    break_counter: u32,

    elapsed_time: Stopwatch,
    // true if pomodoro false otherwise
    is_pomodoro: bool,
}

impl Application for Pomodoro {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self {
            pomodoro_duration: 0,
            pomodoro_counter: 0,
            break_duration: 0,
            break_counter: 0,
            elapsed_time: Stopwatch {
                duration: Duration::default(),
                state: State::Idle,
            },
            is_pomodoro: false,
        },
        Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Pomodoro - Iced")
    }

    fn view(&self) -> Element<Message> {
        // use a simple vertical layout
        const SECOND: u64 = 60;
        const MINUTE : u64 = 60 * SECOND;

        let second = self.elapsed_time.duration.as_secs();
        let font_size = 20;
        let pomodoro_duration_text = format!("{} minutes", self.pomodoro_duration);
        let break_duration_text = format!("{} minutes", self.break_duration);
        let clock_str = format!("{} minutes:{} seconds",
            second / MINUTE,
            second);
        row![
            column![
                row![
                    text("Pomodoro: "),
                ].padding(10)
                .align_items(iced::Alignment::Start),
                row![

                    button("-").on_press(Message::DecrementPomodoroCounter),
                    text(self.pomodoro_counter).size(font_size),
                    button("+").on_press(Message::IncrementPomodoroCounter),
                ].padding(10)
                .align_items(iced::Alignment::Center),
                row![
                    text("Pomodoro duration: "),
                ].padding(10)
                .align_items(iced::Alignment::Start),
                row![
                    button("-").on_press(Message::DecrementPomodoroDuration),
                    text(pomodoro_duration_text).size(font_size),
                    button("+").on_press(Message::IncrementPomodoroDuration),
                ].padding(20)
                .align_items(iced::Alignment::Center),
                row![
                    button("Start").on_press(Message::StartPressed),
                ].align_items(iced::Alignment::Center),
            ],
            column![
                row![
                    text("Break: "),
                ].padding(10)
                .align_items(iced::Alignment::Start),
                row![
                    button("-").on_press(Message::DecrementBreakCounter),
                    text(self.break_counter).size(font_size),
                    button("+").on_press(Message::IncrementBreakCounter),
                ].padding(10)
                .align_items(iced::Alignment::Center),
                row![
                    text("Break duration: "),
                ].padding(10)
                .align_items(iced::Alignment::Start),
                row![
                    button("-").on_press(Message::DecrementBreakDuration),
                    text(break_duration_text).size(font_size),
                    button("+").on_press(Message::IncrementBreakDuration),
                ].padding(20)
                .align_items(iced::Alignment::Center),
                row![
                button("Stop").on_press(Message::StopPressed),
                ].align_items(iced::Alignment::Center),
                row![
                text(clock_str).size(font_size),
                ].align_items(iced::Alignment::Center),
            ],
        ]
        .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncrementPomodoroCounter => {
                self.pomodoro_counter += 1;
            }
            Message::DecrementPomodoroCounter => {
                if self.pomodoro_counter != 0 {
                   self.pomodoro_counter -= 1;
                }
            }
            Message::IncrementPomodoroDuration => {
                self.pomodoro_duration += 1;
            }
            Message::DecrementPomodoroDuration => {
                if self.pomodoro_duration != 0 {
                    self.pomodoro_duration -= 1;
                }
            }

            Message::IncrementBreakCounter => {
                self.break_counter += 1;
            }
            Message::DecrementBreakCounter => {
                if self.break_counter != 0 {
                    self.break_counter -= 1;
                }
            }
            Message::IncrementBreakDuration => {
                self.break_duration += 1;
            }
            Message::DecrementBreakDuration => {
                if self.break_duration != 0 {
                    self.break_duration -= 1;
                }
            }
            Message::StartPressed => {
                self.elapsed_time.state = State::Ticking {
                    last_tick: Instant::now()
                };
                self.is_pomodoro = true;
                println!("Nothing to see here mate");
            }

            Message::Tick(now) => {
                if let State::Ticking { last_tick} = &mut self.elapsed_time.state {
                    self.elapsed_time.duration += now - *last_tick;
                    *last_tick = now;
                }
            }

            Message::StopPressed => {
                self.is_pomodoro = false;
                println!("Nothing to see here mate");
            }
        }

        Command::none()

    }

    //fn subscription(&self) -> Subscription<Message> {
    //}
}
