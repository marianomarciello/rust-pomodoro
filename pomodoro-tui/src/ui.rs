use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph}, prelude::{Layout, Constraint, Direction}, text::Line,
};

use crate::{App, app::EditApp};

use crate::tui::Frame;
use crate::app::AppState;
use ratatui::prelude::Rect;
use std::time::Duration;

struct TopBar {
    line_type: EditApp,
    text: String,
    title: String,
}

pub fn render(app: &App, f: &mut Frame) {
    let layout = layout(f.size());
    f.render_widget(top_bar(app, &TopBar { 
        line_type: EditApp::PomoNum,
        title: String::from(format!("Number of {}", app.pomo_emoji)),
        text: String::from(format!("{}", app.pomo_num)),
    }),layout[0]);

    f.render_widget(top_bar(app, &TopBar { 
        line_type: EditApp::PomoDur,
        title: String::from(format!("{} duration ", app.pomo_emoji)),
        text: String::from(format!("{}", format_duration(&app.pomo_dur))),
    }),layout[1]);

    f.render_widget(top_bar(app, &TopBar { 
        line_type: EditApp::BreakDur,
        title: String::from(format!("{}  duration ", app.timer_emoji)),
        text: String::from(format!("{}", format_duration(&app.break_dur))),
    }),layout[2]);

    f.render_widget(help_paragraph(app),layout[3]);
}

fn layout(area: Rect) -> Vec<Rect> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Max(15), // top bar
            Constraint::Percentage(70), // timer
            Constraint::Max(5), // help paragraph
        ])
        .split(area);

    let top_pomo_num = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Max(5), // num of pomo
            Constraint::Max(5), // pomo dur
            Constraint::Max(5), // break dur
        ])
        .split(layout[0]);

    let help_bar = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(6), // help
        ])
        .split(layout[2]);

    top_pomo_num[..].iter().chain(
        help_bar[..].iter()).chain(
        layout[1..].iter()).copied()
        .collect()
}


fn top_bar<'a>(app: &'a App, bar_element: &'a TopBar) -> Paragraph<'a> {
    if app.edit_app == bar_element.line_type {
        Paragraph::new(bar_element.text.clone())
            .style(Style::new())
            .alignment(Alignment::Center)
            .block(Block::default()
            .title(bar_element.title.clone())
            .title_style(Style::default()).green()
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Thick))
    } else {
        Paragraph::new(bar_element.text.clone())
            .style(Style::new())
            .alignment(Alignment::Center)
            .block(Block::default()
            .title(bar_element.title.clone())
            .title_style(Style::default()).cyan()
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Thick))
    }
}

fn help_paragraph(app: &App) -> Paragraph<'_> {
  let space_action = match app.state {
    AppState::StopPomo | AppState::StopBreak => "start",
    _ => "stop"
  };
  let help_text =
    Line::from(
        vec![
        "space ".into(),
        space_action.dim(),
        " enter".into(),
        " stop".dim(),
        " j".into(),
        " increase".dim(),
        " k".into(),
        " decrease".dim(),
        " q".into(),
        " quit".dim(),
        ]);
  Paragraph::new(help_text)
      .gray()
      .alignment(Alignment::Center)
      .block(Block::default()
          .title("Help")
          .title_style(
              Style::default()
              .cyan())
          .title_alignment(Alignment::Center)
          .border_type(BorderType::Rounded)
          .border_style(Style::default()
              .green())
          .borders(Borders::ALL))
}


fn format_duration(duration: &Duration) -> String {
    format!("{:02}min:{:02}sec",
        duration.as_secs() / 60,
        duration.as_secs() % 60,
        )

}
