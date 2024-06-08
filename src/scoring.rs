use crate::app;

pub fn calculate_wpm(app: &app::App) -> u16 {
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

    let words = correct_words.map(|w| w.0.len() as u16).sum::<u16>() / AVERAGE_ENG_WORD_LEN;

    let elapsed_time_secs = app.game.start_time.expect("failed to get elapsed time").elapsed().as_secs();
    let minutes = elapsed_time_secs as f32 / 60.0;
    let score = words as f32 / minutes;
    score as u16
}

const AVERAGE_ENG_WORD_LEN: u16 = 5;

