use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, Error};

use super::Trie;

#[derive(Clone)]
pub struct TrieLinkedList {
    child: Option<Box<TrieLinkedList>>,
    sibling: Option<Box<TrieLinkedList>>,
    letter: char,
    is_word: bool,
}

fn print_siblings(trie: Option<&TrieLinkedList>) -> String {
    if let Some(t) = trie {
        t.letter.to_string() + &print_siblings(t.sibling.as_deref())
    } else {
        String::new()
    }
}

impl Debug for TrieLinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}=>{}\n \\{}",
            self.letter,
            print_siblings(self.sibling.as_deref()),
            print_siblings(self.child.as_deref())
        ))
    }
}

impl TrieLinkedList {
    pub fn blank<'a>(c: char) -> TrieLinkedList {
        TrieLinkedList {
            child: None,
            sibling: None,
            is_word: false,
            letter: c,
        }
    }

    pub fn from_file(filename: &str) -> Result<TrieLinkedList, Error> {
        let mut dict = TrieLinkedList::blank(' ');
        let file = File::open(filename)?;
        let lines = std::io::BufReader::new(file).lines();
        for word in lines.flatten() {
            dict.add_word(&word);
        }
        Ok(dict)
    }
}

impl Trie for TrieLinkedList {
    fn add_word(&mut self, word: &str) {
        let mut cursor = self;
        let mut chars = word.chars().peekable().enumerate();
        // println!("adding word {}", word);
        while let Some((i,c)) = chars.next() {
            match cursor.child {
                None => {
                    // println!("none child");
                    cursor.child = Some(Box::new(TrieLinkedList::blank(c)));
                    cursor = cursor.child.as_mut().unwrap();
                }
                Some(ref child) if child.letter > c => {
                    // println!("first child greater");
                    // First child here is greater than char to insert so we need to insert in first position and make parent.child point to new node
                    cursor.child = Some(Box::new(TrieLinkedList {
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
                                cursor.sibling = Some(Box::new(TrieLinkedList::blank(c)));
                            }
                            Some(ref sibling) if sibling.letter > c => {
                                cursor.sibling = Some(Box::new(TrieLinkedList {
                                    child: None,
                                    sibling: cursor.sibling.take(),
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

    fn traverse(&self, word: &str) -> Option<&dyn Trie> {
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
        let mut dict = TrieLinkedList::blank(' ');
        dict.add_word("hell");
        dict.add_word("abc");
        dict.add_word("hello");
        dict.add_word("spoon");
        dict.add_word("spoonlike");
        dict.add_word("spoony");
        dict.add_word("spoonmaker");
        dict.add_word("spoonmaking");
        dict.add_word("spoons");
        assert_eq!(dict.traverse("abc").unwrap().is_word(), true);
        assert_eq!(dict.traverse("hello").unwrap().is_word(), true);
        assert_eq!(dict.traverse("he").unwrap().is_word(), false);
        assert_eq!(dict.traverse("fjidso").is_none(), true);
        assert_eq!(dict.traverse("spoonlike").unwrap().is_word(), true);
        assert_eq!(dict.traverse("spoonmaker").unwrap().is_word(), true);
        assert_eq!(dict.traverse("spoonmaking").unwrap().is_word(), true);
        assert_eq!(dict.traverse("spoons").unwrap().is_word(), true);
        assert_eq!(dict.traverse("spoonm").unwrap().is_word(), false);
    }

    #[test]
    fn dict_size() {
        let dict = TrieLinkedList::from_file("./words_alpha.txt").unwrap();
        let mut count = 0;
        let mut stack = Vec::new();
        stack.push(&dict);
        // while stack.len() > 0 {
        //     let dict = stack.pop().unwrap();
        //     count += 1;
        //     stack.append(&mut dict.child.values().collect())
        // }
        // assert_eq!(count, 1027815);
    }

    #[bench]
    fn bench_build_dict(b: &mut Bencher) {
        b.iter(|| TrieLinkedList::from_file("./words_alpha.txt").unwrap())
    }
}
