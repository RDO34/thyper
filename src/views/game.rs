use std::cmp;

use crossterm::event::{KeyCode};
use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::app;
use crate::widgets::centered_rect;

pub fn render(app: &app::App, frame: &mut Frame) {
    let area = frame.size();
    let available_width = area.width as f32 / 100.0 * 80.0;

    let lines = into_lines(&app.game.words, available_width as u16, app.game.position.0 as u8);
    let text: Vec<Vec<Span>> = lines.iter().map(|l| {
        l.iter().map(|w| {
            let mut chars: Vec<_> = w.word.iter()
                .enumerate()
                .map(|(ci, c)| {
                    let input_char_opt = w.input.get(ci);

                    if input_char_opt.is_some() {
                        let input_char = input_char_opt.unwrap();

                        if input_char == c {
                            c.to_string().gray()
                        } else {
                            input_char.to_string().red()
                        }
                    } else {
                        c.to_string().dark_gray()
                    }
                })
            .collect();

            if w.input.len() > w.word.len() {
                let mut rest = w.input[w.word.len()..].iter().map(|c| c.to_string().red()).collect();
                chars.append(&mut rest);
            };

            chars.push(" ".into());

            if w.word_position == app.game.position.0 as u8 {
                chars[app.game.position.1] = chars[app.game.position.1].clone().on_gray();
            };

            chars
        })
        .flatten()
            .collect()
    })
    .collect();

    let text_lines: Vec<Line> = text.iter().map(|l| Line::from(l.clone())).collect();

    frame.render_widget(
        Paragraph::new(text_lines),
        centered_rect(area, 80, 20)
    );
}

pub fn handle_keypress(app: &mut app::App, key_code: KeyCode) {
    match &key_code {
        KeyCode::Char(' ') => {
            app.game.keypress_count += 1;

            if app.game.position.0 < app.game.words.len() - 1 {
                app.game.position.0 += 1;
                app.game.position.1 = 0;
            } else {
                app.end_game();
            };
        },

        KeyCode::Char(c) => {
            app.game.keypress_count += 1;
            let current_word = &mut app.game.words[app.game.position.0];
            current_word.1.push(*c);
            app.game.position.1 += 1;
        },

        KeyCode::Backspace => {
            app.game.keypress_count += 1;
            let current_word = &mut app.game.words[app.game.position.0];

            if current_word.1.len() > 0 {
                current_word.1.pop();

                if app.game.position.1 > 0 {
                    app.game.position.1 -= 1;
                };
            } else if app.game.position.0 > 0 {
                app.game.position.0 -= 1;
                let prev_word = &mut app.game.words[app.game.position.0];
                app.game.position.1 = prev_word.1.len();
            };    
        },

        KeyCode::Esc => {
            if app.mode == app::Mode::Quick {
                app.exit = true;
            } else {
                app.view = app::View::Home;
                app.game.ended = true;
            }
        },

        _ => {},
    }
}

#[derive(Clone)]
struct VisibleWord<'a> {
    pub word: &'a Vec<char>,
    pub input: &'a Vec<char>,
    pub word_position: u8,
}

fn into_lines(words: &Vec<(Vec<char>, Vec<char>)>, width: u16, current_word_position: u8) -> Vec<Vec<VisibleWord>> {
    let mut all_lines: Vec<Vec<VisibleWord>> = vec![];

    words.iter().enumerate().for_each(|(i, w)| {
        let last_line_opt = all_lines.last_mut();
        let visible_word = to_visible_word(w, i as u8);

        if last_line_opt.is_none() {
            let first_line = vec![visible_word];
            all_lines.push(first_line);
        } else {
            let last_line: &mut Vec<VisibleWord<'_>> = last_line_opt.unwrap();

            if u16::from(get_line_width(last_line) + get_word_width(&w.0, &w.1) + 1) > width {
                let next_line = vec![visible_word];
                all_lines.push(next_line);
            } else {
                last_line.push(visible_word)
            };
        };
    });

    let current_line_pos = all_lines.iter()
        .position(|l| {
            l.iter().find(|w| w.word_position == current_word_position).is_some()
        })
    .expect("Failed to find current line position");

    if current_line_pos == 0 {
        vec![
            all_lines[current_line_pos].clone(),
            all_lines[current_line_pos + 1].clone(),
            all_lines[current_line_pos + 2].clone(),
        ]
    } else if current_line_pos == all_lines.len() - 1 {
        vec![
            all_lines[current_line_pos - 2].clone(),
            all_lines[current_line_pos - 1].clone(),
            all_lines[current_line_pos].clone(),
        ]
    } else {
        vec![
            all_lines[current_line_pos - 1].clone(),
            all_lines[current_line_pos].clone(),
            all_lines[current_line_pos + 1].clone(),
        ]
    }
}

fn get_line_width(line: &Vec<VisibleWord>) -> u8 {
    line.iter().map(|w| get_word_width(&w.word, &w.input)).sum::<u8>() + line.len() as u8 - 1
}

fn to_visible_word(word: &(Vec<char>, Vec<char>), pos: u8) -> VisibleWord {
    VisibleWord { word: &word.0, input: &word.1, word_position: pos }
}

fn get_word_width(word: &Vec<char>, word_input: &Vec<char>) -> u8 {
    cmp::max(word.len() as u8, word_input.len() as u8)
}
