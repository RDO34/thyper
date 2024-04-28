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

  let wpm = calculate_wpm(app);
  let score = Line::from(format!("{} WPM", wpm)).bold().alignment(Alignment::Center);
  let stats = Line::from(vec![
    app.game.keypress_count.to_string().gray(),
    "kp".dark_gray(),
  ]).alignment(Alignment::Center);

  let divider = " | ".dark_gray();
  let instructions = Line::from(vec![
    "<enter> ".dark_gray().bold(),
    "next".into(),
    divider.clone(),
    "<o> ".dark_gray().bold(),
    "options".into(),
    divider.clone(),
    "<esc> ".dark_gray().bold(),
    "quit".into(),
  ]).alignment(Alignment::Center);

  let text = vec![
    score,
    "".into(),
    stats,
    "".into(),
    instructions
  ];

  frame.render_widget(
    Paragraph::new(text),
    centered_rect(area,  100, 40)
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

      app.game.position = (0, 0);
      app.game.words = new_words;
      app.game.start_time = Some(Instant::now());
      app.game.started = true;
      app.game.ended = false;
    },

    KeyCode::Char('o') => { app.view = app::View::Options },

    _ => {},
  }
}

fn calculate_wpm(app: &app::App) -> u16 {
  let correct_words = app.game.words.iter().filter(|w| {
    let all_chars_match = w.0.iter().enumerate().all(|(i, c)| {
      let input_opt = w.1.get(i);
      if input_opt.is_some() {
        input_opt.unwrap() == c
      } else {
        false
      }
    });

    all_chars_match && w.0.len() == w.1.len()
  });

  correct_words.map(|w| w.0.len() as u16).sum::<u16>() / AVERAGE_ENG_WORD_LEN
}

const AVERAGE_ENG_WORD_LEN: u16 = 5;
