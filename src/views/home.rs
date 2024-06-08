use crossterm::event::{KeyCode};
use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::app;
use crate::widgets::{centered_rect};

pub fn render(_app: &app::App, frame: &mut Frame) {
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
        KeyCode::Enter => { app.start_game() },
        KeyCode::Char('o') => { app.view = app::View::Options },
        KeyCode::Esc => { app.exit = true },
        _ => {},
    }
}
