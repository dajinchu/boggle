use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Error};

#[derive(Clone, Debug)]
pub struct DictEntry {
    child: Option<Box<DictEntry>>,
    sibling: Option<Box<DictEntry>>,
    letter: char,
    pub is_word: bool,
}

impl DictEntry {
    pub fn blank<'a>(c: char) -> DictEntry {
        DictEntry {
            child: None,
            sibling: None,
            is_word: false,
            letter: c,
        }
    }

    pub fn from_file(filename: &str) -> Result<DictEntry, Error> {
        let mut dict = DictEntry::blank(' ');
        let file = File::open(filename)?;
        let lines = std::io::BufReader::new(file).lines();
        for word in lines.flatten() {
            dict.add_word(word);
        }
        Ok(dict)
    }

    pub fn add_word(&mut self, word: String) {
        let mut cursor = self;
        let mut chars = word.chars().peekable();
        // println!("adding word {}", word);
        while let Some(c) = chars.next() {
            // println!("cursor: {:?}", cursor);
            match cursor.child {
                None => {
                    // println!("none child");
                    cursor.child = Some(Box::new(DictEntry::blank(c)));
                    cursor = cursor.child.as_mut().unwrap();
                }
                Some(ref child) if child.letter > c => {
                    // println!("first child greater");
                    // First child here is greater than char to insert so we need to insert in first position and make parent.child point to new node
                    cursor.child = Some(Box::new(DictEntry {
                        child: None,
                        sibling: cursor.child.take(),
                        letter: c,
                        is_word: false,
                    }));
                    cursor = cursor.child.as_mut().unwrap();
                }
                Some(ref mut child) => {
                    cursor = child.as_mut();
                    while cursor.letter != c {
                        match cursor.sibling {
                            None => {
                                // println!("none sibling");
                                cursor.sibling = Some(Box::new(DictEntry::blank(c)));
                            }
                            Some(ref sibling) if sibling.letter > c => {
                                cursor.sibling = Some(Box::new(DictEntry {
                                    child: None,
                                    sibling: cursor.child.take(),
                                    letter: c,
                                    is_word: false,
                                }));
                            }
                            _ => {}
                        };
                        cursor = cursor.sibling.as_mut().unwrap()
                    }
                }
            }
        }
        cursor.is_word = true;
    }

    pub fn traverse(&self, word: &str) -> Option<&DictEntry> {
        let mut cursor = self;
        for c in word.chars() {
            match cursor.child.as_ref() {
                Some(child) => cursor = child,
                None => return None,
            }
            while cursor.letter != c {
                if cursor.letter > c {
                    return None;
                } else {
                    match cursor.sibling.as_ref() {
                        Some(sibling) => cursor = sibling,
                        None => return None,
                    }
                }
            }
        }
        Some(cursor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn simple_test() {
        let mut dict = DictEntry::blank(' ');
        dict.add_word("hell".to_string());
        dict.add_word("abc".to_string());
        dict.add_word("hello".to_string());
        assert_eq!(dict.traverse("abc").unwrap().is_word, true);
        assert_eq!(dict.traverse("hello").unwrap().is_word, true);
        assert_eq!(dict.traverse("he").unwrap().is_word, false);
        assert_eq!(dict.traverse("fjidso").is_none(), true);
    }

    #[test]
    fn dict_size() {
        let dict = DictEntry::from_file("./words_alpha.txt").unwrap();
        let mut count = 0;
        let mut stack = Vec::new();
        stack.push(&dict);
        // while stack.len() > 0 {
        //     let dict = stack.pop().unwrap();
        //     count += 1;
        //     stack.append(&mut dict.child.values().collect())
        // }
        assert_eq!(count, 1027815);
    }

    #[bench]
    fn bench_build_dict(b: &mut Bencher) {
        b.iter(|| DictEntry::from_file("./words_alpha.txt").unwrap())
    }
}
