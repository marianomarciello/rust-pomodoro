use crate::pomodoro::message::Message;
use iced::executor;
use iced::theme::{self, Theme};
use iced::widget::{button, column, row, text};
use iced::{Application, Command, Element, Subscription};
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
    pomodoro_duration: u64,
    // pomodoro counter
    pomodoro_counter: u32,
    // pomodoro break duration in minutes
    break_duration: u64,

    // timer
    elapsed_time: Stopwatch,
    // true if pomodoro false otherwise
    is_pomodoro: bool,

    // string to print in the gui
    str_pomodoro: String,
}

impl Application for Pomodoro {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                pomodoro_duration: 0,
                pomodoro_counter: 0,
                break_duration: 0,
                elapsed_time: Stopwatch {
                    duration: Duration::default(),
                    state: State::Idle,
                },
                is_pomodoro: true,
                str_pomodoro: "Start a new Pomodoro".to_string(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pomodoro - Iced")
    }

    fn view(&self) -> Element<Message> {
        // use a simple vertical layout
        const MINUTE: u64 = 60;

        let font_size = 20;
        let pomodoro_duration_text = format!("{} minutes", self.pomodoro_duration);
        let break_duration_text = format!("{} minutes", self.break_duration);
        column![
            row![text("Pomodoro: "),]
                .padding(10)
                .align_items(iced::Alignment::Start),
            row![
                button("-").on_press(Message::DecrementPomodoroCounter),
                text(self.pomodoro_counter).size(font_size),
                button("+").on_press(Message::IncrementPomodoroCounter),
            ]
            .padding(10)
            .align_items(iced::Alignment::Center),
            row![
                column![text("Pomodoro duration:")]
                .padding([0, 10])
                .align_items(iced::Alignment::Start),
                column![text("Break duration:")]
                .padding([0, 10])
                .align_items(iced::Alignment::Start),
            ],
            row![
                column![
                button("+").on_press(Message::IncrementPomodoroDuration),
                text(pomodoro_duration_text).size(font_size),
                button("-").on_press(Message::DecrementPomodoroDuration),
                ].padding([0, 10])
                .align_items(iced::Alignment::Start),
                column![
                button("+").on_press(Message::IncrementBreakDuration),
                text(break_duration_text).size(font_size),
                button("-").on_press(Message::DecrementBreakDuration),
                ].padding([0, 85])
                .align_items(iced::Alignment::Start),
            ]
            .padding(10)
            .align_items(iced::Alignment::Center),
            row![
                column![button("Start").on_press(Message::StartPressed)]
                .padding(10)
                .align_items(iced::Alignment::Start),
                column![button("Stop").on_press(Message::StopPressed)]
                .padding(10)
                .align_items(iced::Alignment::Start),
            ],
            row![text(self.str_pomodoro.clone()).size(font_size)]
                .padding(10)
                .align_items(iced::Alignment::Center),
            row![text(format!("{} min {} sec",
                    self.elapsed_time.duration.as_secs() / MINUTE,
                    self.elapsed_time.duration.as_secs() % MINUTE
                )).size(font_size),]
                .padding(10)
                .align_items(iced::Alignment::Center),
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

            Message::IncrementBreakDuration => {
                self.break_duration += 1;
            }
            Message::DecrementBreakDuration => {
                if self.break_duration != 0 {
                    self.break_duration -= 1;
                }
            }
            Message::StartPressed => {
                if self.is_pomodoro && (self.pomodoro_counter <= 0 || self.pomodoro_duration <= 0) {
                    // no pomodoro number set
                    self.str_pomodoro = "Please set a valid Pomodoro number".to_string();
                    return Command::none();
                } else if !self.is_pomodoro && self.break_duration <= 0 {
                    self.str_pomodoro = "Please set a valid Break duration".to_string();
                    return Command::none();
                }
                self.elapsed_time.state = State::Ticking {
                    last_tick: Instant::now(),
                };
            }

            Message::Tick(now) => {
                if let State::Ticking { last_tick } = &mut self.elapsed_time.state {
                    self.elapsed_time.duration += now - *last_tick;
                    *last_tick = now;
                    if self.is_pomodoro {
                        self.str_pomodoro = "Stay focused ^-^".to_string();
                    } else {
                        self.str_pomodoro = "Chill Bro :)".to_string();
                    }
                }

                if self.is_pomodoro &&
                    self.elapsed_time.duration.as_secs() >= self.pomodoro_duration * 60 {
                    // end of a pomodoro
                    println!("end pomodoro");
                    self.elapsed_time.state = State::Idle;
                    self.elapsed_time.duration = Duration::default();
                    self.pomodoro_counter -= 1;
                    self.is_pomodoro = false;
                    self.str_pomodoro = "Start a new Break".to_string();
                } else if !self.is_pomodoro &&
                   self.elapsed_time.duration.as_secs() >= self.break_duration * 60 {
                    println!("end break");
                    // end of break
                    self.elapsed_time.state = State::Idle;
                    self.elapsed_time.duration = Duration::default();
                    self.is_pomodoro = true;
                    self.str_pomodoro = "Start a new Pomodoro".to_string();
                }
            }

            // stop timer
            Message::StopPressed => {
                self.elapsed_time.state = State::Idle;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.elapsed_time.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => {
                iced::time::every(Duration::from_millis(10)).map(Message::Tick)
            }
        }
    }
}
