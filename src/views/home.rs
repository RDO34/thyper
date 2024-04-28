use std::time::{Instant};

use crossterm::event::{KeyCode};
use ratatui::{
  prelude::*,
  widgets::*,
};

use crate::app;
use crate::widgets::{centered_rect};
use crate::words;

pub fn render(app: &app::App, frame: &mut Frame) {
  let area = frame.size();

  let title = Line::from("thyper").italic().bold().alignment(Alignment::Center);

  let divider = " | ".dark_gray();
  let instructions = Line::from(vec![
    "<enter> ".dark_gray().bold(),
    "start".into(),
    divider.clone(),
    "<o> ".dark_gray().bold(),
    "options".into(),
    divider.clone(),
    "<esc> ".dark_gray().bold(),
    "quit".into(),
  ]).alignment(Alignment::Center);

  let text = vec![title, "".into(), instructions];

  frame.render_widget(
    Paragraph::new(text),
    centered_rect(area,  100, 20)
  );
}

pub fn handle_keypress(app: &mut app::App, key_code: KeyCode) {
  match &key_code {
    KeyCode::Enter => {
      app.view = app::View::Game;

      let new_words = words::generate(100)
        .iter()
        .map(|w| (w.chars().collect(), vec![]))
        .collect::<Vec<(Vec<char>, Vec<char>)>>();

      app.game.words = new_words;
      app.game.start_time = Some(Instant::now());
      app.game.started = true;
      app.game.ended = false;
    },

    KeyCode::Char('o') => { app.view = app::View::Options },

    _ => {},
  }
}
