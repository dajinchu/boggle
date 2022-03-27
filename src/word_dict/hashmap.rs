use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Error};

use super::Trie;

#[derive(Clone, Debug)]
pub struct TrieHashMap {
    next: HashMap<char, TrieHashMap>,
    is_word: bool,
}

impl TrieHashMap {
    pub fn blank() -> TrieHashMap {
        TrieHashMap {
            next: HashMap::new(),
            is_word: false,
        }
    }

    pub fn from_file(filename: &str) -> Result<TrieHashMap, Error> {
        let mut dict = TrieHashMap::blank();
        let file = File::open(filename)?;
        let lines = std::io::BufReader::new(file).lines();
        for word in lines.flatten() {
            dict.add_word(&word);
        }
        Ok(dict)
    }
    fn add_word(&mut self, word: &str) {
        let mut cursor = self;
        for c in word.chars() {
            if !cursor.next.contains_key(&c) {
                cursor.next.insert(c, TrieHashMap::blank());
            }
            let entry = cursor.next.get_mut(&c).unwrap();
            cursor = entry;
        }
        cursor.is_word = true;
    }
}

impl Trie for TrieHashMap {
    fn traverse(&self, word: &str) -> Option<&TrieHashMap> {
        let mut cursor = self;
        for c in word.chars() {
            match cursor.next.get(&c) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn simple_test() {
        let mut dict = TrieHashMap::blank();
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
        let dict = TrieHashMap::from_file("./words_alpha.txt").unwrap();
        let mut count = 0;
        let mut stack = Vec::new();
        stack.push(&dict);
        while stack.len() > 0 {
            let dict = stack.pop().unwrap();
            count += 1;
            stack.append(&mut dict.next.values().collect())
        }
        assert_eq!(count, 1027815);
    }

    #[bench]
    fn bench_build_dict(b: &mut Bencher) {
        b.iter(|| TrieHashMap::from_file("./words_alpha.txt").unwrap())
    }
}
