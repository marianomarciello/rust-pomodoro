use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph}, prelude::{Layout, Constraint, Direction}, text::Line,
};
use tui_big_text::BigText;

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

    f.render_widget(motivation_text(app), layout[5]);

    f.render_widget(center_clock(app), layout[8]);

    f.render_widget(help_paragraph(app),layout[3]);
}

fn layout(area: Rect) -> Vec<Rect> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(15), // top bar
            Constraint::Percentage(70), // timer
            Constraint::Min(6), // help paragraph
        ])
        .split(area);

    let top_pomo_num = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(33), // num of pomo
            Constraint::Percentage(33), // pomo dur
            Constraint::Percentage(33), // break dur
        ])
        .split(layout[0]);

    let help_bar = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(6), // help
        ])
        .split(layout[2]);

    let main_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(40), // help
            Constraint::Percentage(60), // help
        ])
        .split(layout[1]);

    let motivation_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(main_area[0]);

    let clock_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(35),
            Constraint::Percentage(65),
        ])
        .split(main_area[1]);

    top_pomo_num[..].iter().chain(
        help_bar[..].iter()).chain(
        motivation_area[..].iter()).chain(
        clock_area[..].iter()).chain(
        layout[2..].iter()).copied()
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

fn center_clock(app: &App) -> BigText {
    let style = match app.state {
        AppState::StopPomo | AppState::StopBreak =>  Style::new().yellow(),
        AppState::RunPomo |  AppState::RunBreak => Style::new().red(),
        AppState::NoMorePomo => Style::new().green(),
        
    };

    let duration = match app.state {
        AppState::StopPomo | AppState::RunPomo =>  format_duration(&app.pomo_dur),
        AppState::StopBreak | AppState::RunBreak =>  format_duration(&app.break_dur),
        AppState::NoMorePomo => String::from(""),
    };

    tui_big_text::BigTextBuilder::default()
        .style(style)
        .lines(vec![
            Line::from(duration)
        ])
        .build().unwrap()
}

fn motivation_text(app: &App) -> Paragraph<'_>{
    let style = match app.state {
        AppState::StopPomo | AppState::StopBreak =>  Style::new().green(),
        AppState::RunPomo |  AppState::RunBreak => Style::new().yellow(),
        AppState::NoMorePomo => Style::new().green(),
        
    };

    let motivation_string = match app.state {
        AppState::StopPomo => "Time to focus, press space",
        AppState::RunPomo =>  "Focus, don't look at me!!",
        AppState::StopBreak => "Time to take a break, press space",
        AppState::RunBreak =>  "Take a break, enjoy your coffe :)",
        AppState::NoMorePomo => "0 Pomodoro Left, add more pomodoros"
    };
    Paragraph::new(motivation_string)
        .alignment(Alignment::Center)
        .block(Block::default()
        .title_style(Style::default()).cyan()
        .title_alignment(Alignment::Center)
        .style(style))
}

fn help_paragraph(app: &App) -> Paragraph<'_> {
  let space_action = match app.state {
    AppState::StopPomo | AppState::StopBreak => "start",
    _ => "stop"
  };
  let next_element = match app.edit_app {
      EditApp::Nothing => "edit pomodoro's number",
      EditApp::PomoNum => "edit pomodoro's duration",
      EditApp::PomoDur => "edit break's duration",
      EditApp::BreakDur => "No action",
  };


  let help_text =
    Line::from(
        vec![
        "space ".into(),
        space_action.dim(),
        " tab => ".into(),
        next_element.yellow() ,
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
