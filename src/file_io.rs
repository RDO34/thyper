use std::fs;
use std::path::PathBuf;

use dirs::data_local_dir;

pub enum File {
    Scores,
    Words,
    Options,
}

impl File {
    fn path(&self) -> PathBuf {
        let mut path = data_local_dir().expect("unable to find local data dir");
        let file_name = match self {
            File::Scores => "scores.json",
            File::Words => "words.json",
            File::Options => "options.json",
        };
        path.push("thyper");
        path.push(file_name);
        path
    }
}

pub fn read_file(file: File) -> String {
    let file_path = file.path();
    
    fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("cannot find {}: install may be corrupted", file_path.display()))
}

pub fn write_file(file: File, data: &str) {
    let file_path = file.path();

    fs::write(&file_path, data)
        .unwrap_or_else(|_| panic!("failed to write to {}: install may be corrupted", file_path.display()));
}
