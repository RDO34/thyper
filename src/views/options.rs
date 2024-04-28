use crossterm::event::{KeyCode};
use ratatui::{
  prelude::*,
  widgets::*,
};

use crate::app;

pub fn render(app: &app::App, frame: &mut Frame) {
  let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(frame.size());

  let area = frame.size();
  frame.render_widget(
    Paragraph::new("Options Top")
        .block(Block::new().borders(Borders::ALL)),
    layout[0]);
  frame.render_widget(
    Paragraph::new("Options Bottom")
        .block(Block::new().borders(Borders::ALL)),
    layout[1]);
}

pub fn handle_keypress(app: &mut app::App, key_code: KeyCode) {
  match &key_code {
    KeyCode::Enter => { app.view = app::View::Game },
    _ => {},
  }
}
