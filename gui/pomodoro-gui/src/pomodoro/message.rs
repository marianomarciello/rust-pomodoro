use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPomodoroCounter,
    DecrementPomodoroCounter,
    IncrementPomodoroDuration,
    DecrementPomodoroDuration,
    IncrementBreakCounter,
    DecrementBreakCounter,
    IncrementBreakDuration,
    DecrementBreakDuration,
    StartPressed,
    StopPressed,
    Tick(Instant),
}
