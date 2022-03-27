
use std::fs::File;
use std::io::{BufRead, Error};

use super::Trie;

#[derive(Clone, Debug)]
pub struct TrieVec {
    next: Box<WordDict>,
    is_word: bool,
}

pub type WordDict = [Option<TrieVec>; 26];

impl TrieVec {
    pub fn blank() -> TrieVec {
        TrieVec {
            next: Box::new([
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ]),
            is_word: false,
        }
    }

    pub fn from_file(filename: &str) -> Result<TrieVec, Error> {
        let mut dict = TrieVec::blank();
        let file = File::open(filename)?;
        let lines = std::io::BufReader::new(file).lines();
        for word in lines.flatten() {
            dict.add_word(&word);
        }
        Ok(dict)
    }
}
impl Trie for TrieVec {
    fn add_word(&mut self, word: &str) {
        let mut cursor = self;
        for c in word.chars() {
            if cursor.next[char_to_index(c)].is_none() {
                cursor.next[char_to_index(c)] = Some(TrieVec::blank());
            }
            let entry = cursor.next.as_mut()[char_to_index(c)]
                .as_mut()
                .expect("failed to build word dict");
            cursor = entry;
        }
        cursor.is_word = true;
    }

    fn traverse(&self, word: &str) -> Option<&dyn Trie> {
        let mut cursor = self;
        for c in word.chars() {
            match cursor.next[char_to_index(c)].as_ref() {
                None => return None,
                Some(words) => {
                    cursor = words;
                }
            }
        }
        Some(cursor)
    }

    fn is_word(&self) -> bool {
        self.is_word
    }
}


fn char_to_index(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn simple_test() {
        let mut dict = TrieVec::blank();
        dict.add_word("hell");
        dict.add_word("abc");
        dict.add_word("hello");
        assert_eq!(dict.traverse("abc").unwrap().is_word(), true);
        assert_eq!(dict.traverse("hello").unwrap().is_word(), true);
        assert_eq!(dict.traverse("he").unwrap().is_word(), false);
        assert_eq!(dict.traverse("fjidso").is_none(), true);
    }

    #[test]
    fn dict_size() {
        let dict = TrieVec::from_file("./words_alpha.txt").unwrap();
        let mut count = 0;
        let mut stack = Vec::new();
        stack.push(&dict);
        while stack.len() > 0 {
            let dict = stack.pop().unwrap();
            count +=1;
            stack.append(&mut dict.next.iter().flatten().collect())
        }
        assert_eq!(count, 1027815);
    }

    #[bench]
    fn bench_build_dict(b: &mut Bencher) {
        b.iter(|| TrieVec::from_file("./words_alpha.txt").unwrap())
    }

}