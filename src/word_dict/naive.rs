// Not even a trie. Just a sorted list we go through

use std::fs::File;
use std::io::{BufRead, Error};

#[derive(Clone, Debug)]
pub struct TrieNaive {
    vec: Vec<String>,
}

impl TrieNaive {
    pub fn blank() -> TrieNaive {
        TrieNaive { vec: Vec::new() }
    }

    pub fn from_file(filename: &str) -> Result<TrieNaive, Error> {
        let mut dict = TrieNaive::blank();
        let file = File::open(filename)?;
        let lines = std::io::BufReader::new(file).lines();
        for word in lines.flatten() {
            dict.add_word(&word);
        }
        dict.vec.sort_unstable();
        Ok(dict)
    }

    fn add_word(&mut self, word: &str) {
        self.vec.push(word.to_string());
    }

    pub fn is_word(&self, word: &str) -> bool {
        self.vec.binary_search(&word.to_string()).is_ok()
    }

    pub fn is_prefix(&self, word: &str) -> bool {
        self.vec
            .binary_search_by(|w| {
                if w.starts_with(word) {
                    std::cmp::Ordering::Equal
                } else {
                    w.cmp(&word.to_string())
                }
            })
            .is_ok()
    }
}
