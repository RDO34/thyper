use rand::Rng;

use crate::app;
use crate::file_io;

pub fn generate(num_words: u16) -> Vec<String> {
    let possible_words = load_words_data();
    let sequence = create_random_sequence(possible_words.len(), num_words);
    sequence.iter().map(|i| possible_words[*i].clone()).collect::<Vec<_>>()
}

fn load_words_data() -> Vec<String> {
    let words_file = file_io::read_file(file_io::File::Words);   
    let words: Vec<String> = serde_json::from_str(&words_file)
        .expect("unable to parse words data");

    words 
}

fn create_random_sequence(num_words: usize, size: u16) -> Vec<usize> {
    let mut rng = rand::thread_rng();

    (0..size).collect::<Vec<_>>()
        .iter()
        .map(|_| rng.gen_range(0..num_words))
        .collect::<Vec<_>>()
}

pub fn apply_options(app: &app::App, str: &str) -> String {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..=100);

    if app.options.capitals {
        if n > 99 {
            return str.to_uppercase();
        }

        if n > 90 {
            return capitalise_first(str);
        }
    }

    if app.options.punctuation {
        if n > 80 && n <= 90 {
            return punctuate(str, n);
        }

    }

    if app.options.numbers {
        if n > 70 && n <= 80 {
            return rng.gen_range(0..=1000).to_string();
        }
    } 

    str.to_lowercase().to_string()
}

fn capitalise_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(ch) => ch.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn punctuate(s: &str, n: u16) -> String {
    match n {
        81 => format!("({})", s),
        82 => format!("{}.", s),
        83 => format!("{},", s),
        84 => format!("{}!", s),
        85 => format!("{}?", s),
        86 => format!("{}:", s),
        87 => format!("{};", s),
        88 => format!("[{}]", s),
        89 => format!("{{{}}}", s),
        _ => format!("<{}>", s),
    }.to_lowercase()
}

