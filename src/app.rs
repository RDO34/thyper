use std::io;
use std::time::{Instant};

use crossterm::event::{self, KeyEventKind};
use ratatui::{
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::persistence;
use crate::scoring;
use crate::tui::Tui;
use crate::views;
use crate::words;

#[derive(Debug, Default, PartialEq)]
pub enum Mode {
    #[default]
    Normal,
    Quick,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    pub time: Option<u64>,
    pub words: u16,
    pub punctuation: bool,
    pub numbers: bool,
    pub capitals: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            time: Some(60),
            words: 100,
            punctuation: false,
            numbers: false,
            capitals: false,
        }
    }
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
pub struct Score {
    pub wpm: u16,
}

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
    pub score: Score,
    pub persisted_scores: Option<persistence::PersistedScores>
}

#[derive(Debug, Default)]
pub struct App {
    pub mode: Mode,
    pub options: Options,
    pub view: View,
    pub game: Game,
    pub exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut Tui, quick_mode: &bool) -> (io::Result<()>, u16) {
        self.options = persistence::get_persisted_options();

        if *quick_mode {
            self.mode = Mode::Quick;
            self.start_game();
        }

        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))
                .expect("failed to render view");
            self.handle_events();
            self.handle_timers();
        }

        (Ok(()), self.game.score.wpm)
    }

    pub fn start_game(&mut self) {
        self.view = View::Game;

        let new_words = words::generate(self.options.words)
            .iter()
            .map(|w| words::apply_options(self, w))
            .map(|w| (w.chars().collect(), vec![]))
            .collect::<Vec<(Vec<char>, Vec<char>)>>();

        self.game.keypress_count = 0;
        self.game.position = (0, 0);
        self.game.words = new_words;
        self.game.start_time = Some(Instant::now());
        self.game.started = true;
        self.game.ended = false;
    }

    pub fn end_game(&mut self) {
        self.game.ended = true;
        self.game.started = false;
        self.view = View::Score;
        self.game.score.wpm = scoring::calculate_wpm(self);
        self.game.persisted_scores = Some(persistence::get_updated_persisted_scores(self.game.score.wpm));

        if self.mode == Mode::Quick {
            self.exit = true;
        }
    }

    pub fn save_options(&self) {
        persistence::persist_options(&self.options);        
    }

    fn render_frame(&self, frame: &mut Frame) {
        match &self.view {
            View::Home => views::home::render(&self, frame),
            View::Game => views::game::render(&self, frame),
            View::Score => views::score::render(&self, frame),
            View::Options => views::options::render(&self, frame),
        }
    }

    fn handle_events(&mut self) {
        if event::poll(std::time::Duration::from_millis(16)).expect("failed to poll event") {
            if let Ok(event::Event::Key(key)) = event::read() {
                if key.kind == KeyEventKind::Press {
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

    fn handle_timers(&mut self) {
        if self.game.start_time.is_some() && self.options.time.is_some() && self.game.started && !self.game.ended {
            let elapsed_time_secs = self.game.start_time.unwrap().elapsed().as_secs();
            if elapsed_time_secs > self.options.time.expect("failed to get time option") {
                self.end_game();
            }
        }
    }
}

