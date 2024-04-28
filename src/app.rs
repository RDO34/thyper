use std::io;
use std::time::{Instant};

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
  prelude::*,
  widgets::*,
};

use crate::tui::Tui;
use crate::views;

#[derive(Debug, Default, PartialEq)]
pub enum Mode {
  #[default]
  Time,
  Words,
  Quick,
}

#[derive(Debug, Default)]
pub struct Options {
  time: u8,
  punctuation: bool,
  numbers: bool,
}

#[derive(Debug, Default)]
pub enum View {
  #[default]
  Home,
  Game,
  Options,
  Score,
}

type Position = (usize, usize);
type Word = (Vec<char>, Vec<char>);

#[derive(Debug, Default)]
pub struct Game {
  pub words: Vec<Word>,
  pub position: Position,
  pub incorrect_count: u16,
  pub missed_count: u16,
  pub extra_count: u16,
  pub keypress_count: u16,
  pub started: bool,
  pub ended: bool,
  pub start_time: Option<Instant>,
}

#[derive(Debug, Default)]
pub struct App {
  pub mode: Mode,
  pub options: Options,
  pub view: View,
  pub game: Game,
  exit: bool,
}

impl App {
  pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
    while !self.exit {
      terminal.draw(|frame| self.render_frame(frame))?;
      self.handle_events();
      run_timer(self);
    }
    Ok(())
  }

  fn render_frame(&self, frame: &mut Frame) {
    // let area = frame.size();
    // frame.render_widget(
    //   Paragraph::new(format!("{:?}", self)),
    //   area,
    // );
    // frame.render_widget(
    //   Paragraph::new(format!("{:?} {:?} {:?} {:?}", self.game.start_time.is_none(), self.mode == Mode::Time, self.game.started, !self.game.ended)),
    //   area,
    // );
    match &self.view {
      View::Home => views::home::render(&self, frame),
      View::Game => views::game::render(&self, frame),
      View::Score => views::score::render(&self, frame),
      View::Options => views::options::render(&self, frame),
    }
  }

  fn handle_events(&mut self) {
    if event::poll(std::time::Duration::from_millis(16)).expect("Failed to poll event") {
      if let Ok(event::Event::Key(key)) = event::read() {
        if key.kind == KeyEventKind::Press {
          // always allow exit with escape keypress
          if key.code == KeyCode::Esc {
            self.exit = true;
          } else {
            match &self.view {
              View::Home => views::home::handle_keypress(self, key.code),
              View::Game => views::game::handle_keypress(self, key.code),
              View::Score => views::score::handle_keypress(self, key.code),
              View::Options => views::options::handle_keypress(self, key.code),
            }
          }
        }
      }
    }
  }
}

fn run_timer(app: &mut App) {
  if app.game.start_time.is_some() && app.mode == Mode::Time && app.game.started && !app.game.ended {
    let elapsed_time_secs = app.game.start_time.unwrap().elapsed().as_secs();
    if elapsed_time_secs > 60 {
      app.game.ended = true;
      app.game.started = false;
      app.view = View::Score;
    }
  }
}
