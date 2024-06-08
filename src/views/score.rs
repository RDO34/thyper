use crossterm::event::{KeyCode};
use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::app;
use crate::widgets::{centered_rect};

pub fn render(app: &app::App, frame: &mut Frame) {
    let area = frame.size();

    let score = Line::from(vec![
        app.game.score.wpm.to_string().gray(),
        "wpm".dark_gray(),
        " / ".into(),
        app.game.keypress_count.to_string().gray(),
        "kp".dark_gray(),
    ]).bold().alignment(Alignment::Center);

    let persisted = Line::from(vec![
        "pb ".dark_gray(),
        app.game.persisted_scores.as_ref().unwrap().pb.to_string().gray(),
        " / ".into(),
        "avg ".dark_gray(),
        app.game.persisted_scores.as_ref().unwrap().avg.to_string().gray(),
    ]).alignment(Alignment::Center);

    let divider = " | ".dark_gray();
    let instructions = Line::from(vec![
        "<enter> ".dark_gray().bold(),
        "next".into(),
        divider.clone(),
        "<esc> ".dark_gray().bold(),
        "menu".into(),
    ]).alignment(Alignment::Center);

    let text = vec![
        score,
        "".into(),
        persisted,
        "".into(),
        instructions,
    ];

    frame.render_widget(
        Paragraph::new(text),
        centered_rect(area,  100, 40)
    );
}

pub fn handle_keypress(app: &mut app::App, key_code: KeyCode) {
    match &key_code {
        KeyCode::Enter => { app.start_game() },
        KeyCode::Esc => { app.view = app::View::Home },
        _ => {},
    }
}

