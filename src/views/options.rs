use crossterm::event::{KeyCode};
use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::app;
use crate::widgets::{centered_rect};

pub fn render(app: &app::App, frame: &mut Frame) {
    let area = frame.size();
    let width = (24 as f32 / 100 as f32) * area.width as f32;

    let options = &app.options;

    let time = Line::from(vec![
        "<t> ".dark_gray(),
        "time        ".gray(),
        time_to_text(options.time).into(),
    ]).alignment(Alignment::Center);

    let words = Line::from(vec![
        "<w> ".dark_gray(),
        "words       ".gray(),
        options.words.to_string().into(),
    ]).alignment(Alignment::Center);

    let punctuation = Line::from(vec![
        "<p> ".dark_gray(),
        "punctuation ".gray(),
        bool_to_text(&options.punctuation).into(),         
    ]).alignment(Alignment::Center);

    let numbers = Line::from(vec![
        "<n> ".dark_gray(),
        "numbers     ".gray(),
        bool_to_text(&options.numbers).into(),         
    ]).alignment(Alignment::Center);

    let capitals = Line::from(vec![
        "<c> ".dark_gray(),
        "captitals   ".gray(),
        bool_to_text(&options.capitals).into(),         
    ]).alignment(Alignment::Center);

    let instructions = Line::from(vec![
        "<esc> ".dark_gray().bold(),
        "menu".into(),
    ]).alignment(Alignment::Center);

    let text = vec![
        time,
        words,
        punctuation,
        numbers,
        capitals,
        "".into(),
        instructions,
    ];

    frame.render_widget(
        Paragraph::new(text),
        centered_rect(area, width as u16, 40)
    );
}

pub fn handle_keypress(app: &mut app::App, key_code: KeyCode) {
    match &key_code {
        KeyCode::Esc => { app.view = app::View::Home },

        KeyCode::Char('t') => {
            let mut i: usize = TIMES
                .iter()
                .position(|t| t == &app.options.time)
                .expect("unknown time option") + 1;

            if i > TIMES.len() - 1 {
                i = 0;
            }
            
            app.options.time = TIMES[i];
            app.save_options();
        },

        KeyCode::Char('w') => {
            let mut i: usize = WORD_COUNTS
                .iter()
                .position(|w| w == &app.options.words)
                .expect("unknown word count option") + 1;

            if i > WORD_COUNTS.len() - 1 {
                i = 0;
            }

            app.options.words = WORD_COUNTS[i];
            app.save_options();
        },


        KeyCode::Char('p') => {
            app.options.punctuation = !app.options.punctuation;
            app.save_options();
        },

        KeyCode::Char('n') => { 
            app.options.numbers = !app.options.numbers;
            app.save_options();
        },

        KeyCode::Char('c') => {
            app.options.capitals = !app.options.capitals;
            app.save_options();
        },

        _ => {},
    }
}

fn bool_to_text(b: &bool) -> &str {
    match b {
        true => "[v]",
        false => "[ ]",
    }
}

fn time_to_text(t: Option<u64>) -> String {
    match t {
        Some(n) => format!("{}s", n),
        None => "none".to_string(),
    }
}

static TIMES: [Option<u64>; 4] = [None, Some(30), Some(60), Some(120)];
static WORD_COUNTS: [u16; 4] = [50, 100, 200, 500];

