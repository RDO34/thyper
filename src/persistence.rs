use serde::{Deserialize, Serialize};

use crate::app;
use crate::file_io;

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistedScores {
    pub pb: u16,
    pub avg: f32,
    pub total: u32,
}

pub fn get_persisted_scores() -> PersistedScores {
    let scores_file = file_io::read_file(file_io::File::Scores);
    let scores: PersistedScores = serde_json::from_str(&scores_file)
        .expect("failed to parse persisted score");

    scores
}

pub fn get_updated_persisted_scores(wpm: u16) -> PersistedScores {
    let persisted_scores = get_persisted_scores();
    let has_persisted_scores = persisted_scores.pb != 0;

    let new_avg = if has_persisted_scores {
        (persisted_scores.avg * persisted_scores.total as f32 + wpm as f32) / (persisted_scores.total + 1) as f32
    } else {
        wpm as f32
    };

    let pb = if wpm > persisted_scores.pb { wpm } else { persisted_scores.pb };

    let new_scores = PersistedScores{
        pb: pb,
        avg: new_avg,
        total: persisted_scores.total + 1,
    };

    let new_data = serde_json::to_string(&new_scores).expect("failed to persist scores");
    file_io::write_file(file_io::File::Scores, &new_data);

    new_scores
}

pub fn persist_options(options: &app::Options) {
    let new_options = serde_json::to_string(options).expect("failed to serialise options");
    file_io::write_file(file_io::File::Options, &new_options);
}

pub fn get_persisted_options() -> app::Options {
    let options_file = file_io::read_file(file_io::File::Options);
    let options: app::Options = serde_json::from_str(&options_file)
        .expect("failed to parse persisted options");

    options
}


