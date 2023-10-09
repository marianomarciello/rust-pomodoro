pub mod app;
use app::App;

pub mod tui;
use tui::Tui;

pub mod ui;

pub mod event;
use event::{Event, EventHandler};

pub mod update;
use update::{update, increase_timer};

use ratatui::prelude::{CrosstermBackend, Terminal};

use anyhow::Result;

fn main() -> Result<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(25);
    let mut tui = Tui::new(terminal, events);

    tui.init()?;

    while !app.should_quit {
        // render the user interface
        tui.draw(&mut app)?;

        // handle events
        match tui.events.next()? {
            Event::Tick => increase_timer(&mut app),
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
// TODO https://ratatui-org.github.io/ratatui-book/tutorial/json-editor/index.html
