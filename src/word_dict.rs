use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Error};

#[derive(Clone, Debug)]
pub struct DictEntry {
    next: WordDict,
    pub is_word: bool,
}

pub type WordDict = HashMap<char, DictEntry>;

impl DictEntry {
    pub fn blank() -> DictEntry {
        DictEntry {
            next: HashMap::new(),
            is_word: false,
        }
    }

    pub fn from_file(filename: &str) -> Result<DictEntry, Error> {
        let mut dict = DictEntry::blank();
        let file = File::open(filename)?;
        let lines = std::io::BufReader::new(file).lines();
        for word in lines.flatten() {
            dict.add_word(word);
        }
        Ok(dict)
    }

    pub fn add_word(&mut self, word: String) {
        let mut cursor = self;
        for c in word.chars() {
            if !cursor.next.contains_key(&c) {
                cursor.next.insert(c, DictEntry::blank());
            }
            let entry = cursor.next.get_mut(&c).unwrap();
            cursor = entry;
        }
        cursor.is_word = true;
    }

    pub fn traverse(&self, word: &str) -> Option<&DictEntry> {
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
}


fn char_to_index(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn dict_size() {
        let dict = DictEntry::from_file("./words_alpha.txt").unwrap();
        let mut count = 0;
        let mut stack = Vec::new();
        stack.push(&dict);
        while stack.len() > 0 {
            let dict = stack.pop().unwrap();
            count +=1;
            stack.append(&mut dict.next.values().collect())
        }
        assert_eq!(count, 1027815);
    }

    #[bench]
    fn bench_build_dict(b: &mut Bencher) {
        b.iter(|| DictEntry::from_file("./words_alpha.txt").unwrap())
    }

}
