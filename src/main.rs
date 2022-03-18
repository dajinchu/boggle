use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone,Debug)]
struct DictEntry {
    next: Box<WordDict>,
    is_word: bool,
}

impl DictEntry {
    fn blank() -> DictEntry {
        DictEntry {
            next: Box::new([None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None]),
            is_word: false,
        }
    }

    fn is_word(&self, word: String) -> bool {
        let mut cursor = self;
        for c in word.chars() {
            match cursor.next[char_to_index(c)].as_ref() {
                None => return false,
                Some(words)=> {
                    cursor = words;
                }
            }
        }
        cursor.is_word
    }
}

type WordDict = [Option<DictEntry>; 26];

fn char_to_index(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

fn add_word(word: String, words: &mut DictEntry) {
    let mut cursor = words;
    for c in word.chars() {
        if let None = cursor.next[char_to_index(c)] {
            cursor.next[char_to_index(c)] = Some(DictEntry::blank());
        }
        let entry = cursor.next.as_mut()[char_to_index(c)].as_mut().expect("failed to build word dict");
        cursor = entry;
    }
    cursor.is_word = true;
}

fn main() {
    let mut words = DictEntry::blank();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./words_alpha.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(word) = line {
                add_word(word, &mut words);
            }
        }
    }
    println!("{:?}", words.is_word("jkl".to_string()));
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
