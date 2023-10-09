use crate::app::{App, AppState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Left | KeyCode::Char('j') => app.increment(),
        KeyCode::Right | KeyCode::Char('k') => app.decrement(),
        KeyCode::Tab => app.next_edit(),
        KeyCode::Char(' ') => app.toggle_start_stop(),
        _ => {}
    }
}

pub fn increase_timer(app: &mut App) {
    // increase timer only if we are runing
    match app.state {
        AppState::RunPomo => {
            app.update_timer();
        }
        AppState::RunBreak => {
            app.update_break_timer();
        }
        _ => {}
    }
}
