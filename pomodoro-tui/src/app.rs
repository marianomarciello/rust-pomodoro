use std::time::{Duration, Instant};

/**
 * Constants
 */
// MIN_DURATION: 1 min
const MIN_DURATION: Duration = Duration::from_secs(1 * 60);
const DEFAULT_POMO_DUR: Duration = Duration::from_secs(50 * 60);
const DEFAULT_BREAK_DUR: Duration = Duration::from_secs(15 * 60);

#[derive(Debug, Default)]
pub enum AppState {
    #[default]
    StopPomo,
    RunPomo,
    StopBreak,
    RunBreak,
    NoMorePomo,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum EditApp {
    #[default]
    Nothing = 0,
    PomoNum = 1,
    PomoDur = 2,
    BreakDur = 3
}

// App state
#[derive(Debug)]
pub struct App {
    pub edit_app: EditApp,
    pub pomo_num: u64,
    pub pomo_dur: Duration,
    // backup of `pomo_dur` so we can set this value at the end of the timer
    pub pomo_dur_bk: Duration,
    pub break_dur: Duration,
    // backup of `break_dur` so we can set this value at the end of the timer
    pub break_dur_bk: Duration,
    pub counter: i64,
    pub should_quit: bool,
    pub pomo_emoji: String,
    pub timer_emoji: String,
    start_time: Instant,
    pub state: AppState,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}


impl App {
    // constructor
    pub fn new() -> Self {
        App { 
            edit_app: EditApp::Nothing,
            pomo_emoji: emojis::get_by_shortcode("tomato").unwrap().to_string(),
            timer_emoji: emojis::get_by_shortcode("timer_clock").unwrap().to_string(),
            pomo_num: 1,
            pomo_dur: DEFAULT_POMO_DUR,
            pomo_dur_bk: DEFAULT_POMO_DUR,
            break_dur: DEFAULT_BREAK_DUR,
            break_dur_bk: DEFAULT_BREAK_DUR,
            state: AppState::StopPomo,
            start_time: Instant::now(),
            counter: 0,
            should_quit: false
        }
    }

    /**
     * Pub methods
     */
    // handles the tick event to the terminal
    pub fn tick(&self) {
    }

    // set running to false to quit the app
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn next_edit(&mut self) {
        self.edit_app = match self.edit_app {
            EditApp::Nothing => EditApp::PomoNum,
            EditApp::PomoNum => EditApp::PomoDur,
            EditApp::PomoDur => EditApp::BreakDur,
            EditApp::BreakDur => EditApp::Nothing,
        }
    }

    pub fn increment(&mut self) {
        match self.edit_app {
            EditApp::PomoNum => self.increment_pomo_num(),
            EditApp::PomoDur => self.increment_pomo_dur(),
            EditApp::BreakDur => self.increment_break_dur(),
            EditApp::Nothing => (),
        }
    }

    pub fn decrement(&mut self) {
        match self.edit_app {
            EditApp::PomoNum => self.decrement_pomo_num(),
            EditApp::PomoDur => self.decrement_pomo_dur(),
            EditApp::BreakDur => self.decrement_break_dur(),
            EditApp::Nothing => (),
        }
    }

    pub fn toggle_start_stop(&mut self) {
        match self.state {
            AppState::RunPomo => { 
                self.state = AppState::StopPomo;
            },
            AppState::StopPomo => { 
                if self.pomo_dur == Duration::ZERO {
                    // TODO: ring a bell
                    // decrease pomo_num
                    self.decrement_pomo_num();
                    self.pomo_dur = self.pomo_dur_bk;
                    self.break_dur = self.break_dur_bk;
                    self.start_time = Instant::now();
                    self.state = AppState::StopBreak;
                } else {
                    self.start_time = Instant::now();
                    self.state = AppState::RunPomo;
                }
                if self.pomo_num == 0 {
                    self.state = AppState::NoMorePomo
                }
            },
            AppState::RunBreak => {
                self.state = AppState::StopBreak;
            },
            AppState::StopBreak => {
                if self.break_dur == Duration::ZERO {
                    self.break_dur = self.break_dur_bk;
                    self.pomo_dur = self.pomo_dur_bk;
                    self.start_time = Instant::now();

                    self.state = AppState::StopPomo;
                } else {
                    self.start_time = Instant::now();
                    self.state = AppState::RunBreak;
                }
            }
            AppState::NoMorePomo => {
                if self.pomo_num > 0 {
                    self.start_time = Instant::now();
                    self.state = AppState::RunPomo
                }
            }
        }
    }

    pub fn update_timer(&mut self, ) {
       let now = Instant::now();
       let elapsed = now - self.start_time;
       if let Some(res) = self.pomo_dur.checked_sub(elapsed) {
           self.start_time = now;
           self.pomo_dur = res;
       } else {
           self.pomo_dur = Duration::ZERO;
           self.state = AppState::StopPomo;
           self.toggle_start_stop();
       }
    }

    pub fn update_break_timer(&mut self) {
       let now = Instant::now();
       let elapsed = now - self.start_time;
       if let Some(res) = self.break_dur.checked_sub(elapsed) {
           self.start_time = now;
           self.break_dur = res;
       } else {
           self.break_dur = Duration::ZERO;
           self.state = AppState::StopBreak;
           self.toggle_start_stop();
       }
    }

    /**
     * Priv methods
     */

    fn decrement_pomo_dur(&mut self) {
        if let Some(res) = self.pomo_dur.checked_sub(Duration::from_secs(60)) {
            if res >= MIN_DURATION {
                self.pomo_dur = res;
            }
        }
    }

    fn increment_pomo_dur(&mut self) {
        if let Some(res) = self.pomo_dur.checked_add(Duration::from_secs(60)) {
                self.pomo_dur = res;
        }
    }

    fn decrement_break_dur(&mut self) {
        if let Some(res) = self.break_dur.checked_sub(Duration::from_secs(60)) {
            if res >= MIN_DURATION {
                self.break_dur = res;
            }
        }
    }

    fn increment_break_dur(&mut self) {
        if let Some(res) = self.break_dur.checked_add(Duration::from_secs(60)) {
            self.break_dur = res;
        }
    }

    fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    fn increment_pomo_num(&mut self) {
        if let Some(res) = self.pomo_num.checked_add(1) {
            self.pomo_num = res;
        }
    }
    fn decrement_pomo_num(&mut self) {
        if let Some(res) = self.pomo_num.checked_sub(1) {
            self.pomo_num = res;
        }
    }
}

mod test {
    use crate::App;

    #[test]
    fn test_app_increment_counter() {
        let mut app = App::default();
        app.increment_pomo_num();
        assert_eq!(app.pomo_num, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::default();
        app.decrement_counter();
        assert_eq!(app.counter, -1);
    }
}
