use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPomodoroCounter,
    DecrementPomodoroCounter,
    IncrementPomodoroDuration,
    DecrementPomodoroDuration,
    IncrementBreakDuration,
    DecrementBreakDuration,
    StartPressed,
    StopPressed,
    Tick(Instant),
}
