use std::fs;
use rand::Rng;

pub fn generate(num_words: u8) -> Vec<String> {
  let mut words: Vec<String> = vec![];
  let possible_words = load_words_data();

  let sequence = create_random_sequence(possible_words.len(), num_words);

  sequence.iter().map(|i| possible_words[*i].clone()).collect::<Vec<_>>()
}

fn load_words_data() -> Vec<String> {
  let words_file = fs::read_to_string("static/words.json")
    .expect("Unable to read data");
    
  let words: Vec<String> = serde_json::from_str(&words_file)
    .expect("Unable to parse words data");

  words
}

fn create_random_sequence(num_words: usize, size: u8) -> Vec<usize> {
  let mut rng = rand::thread_rng();
  let _range = 0..num_words;

  (0..size).collect::<Vec<_>>()
    .iter()
    .map(|_| rng.gen_range(0..num_words))
    .collect::<Vec<_>>()
}