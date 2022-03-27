use std::cell::Cell;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, Error};

use typed_arena::Arena;

use super::Trie;

#[derive(Clone)]
pub struct TrieLinkedListArena<'a> {
    child: Cell<Option<&'a TrieLinkedListArena<'a>>>,
    sibling: Cell<Option<&'a TrieLinkedListArena<'a>>>,
    letter: char,
    is_word: bool,
}

fn print_siblings(trie: Option<&TrieLinkedListArena>) -> String {
    if let Some(t) = trie {
        t.letter.to_string() + &print_siblings(t.sibling.get())
    } else {
        String::new()
    }
}

impl Debug for TrieLinkedListArena<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}=>{}\n \\{}",
            self.letter,
            print_siblings(self.sibling.get()),
            print_siblings(self.child.get())
        ))
    }
}

impl<'a> TrieLinkedListArena<'a> {
    pub fn blank(c: char) -> TrieLinkedListArena<'a> {
        TrieLinkedListArena {
            child: Cell::new(None),
            sibling: Cell::new(None),
            is_word: false,
            letter: c,
        }
    }

    pub fn from_file(
        filename: &str,
        arena: &'a Arena<TrieLinkedListArena<'a>>,
    ) -> Result<TrieLinkedListArena<'a>, Error> {
        let dict = TrieLinkedListArena::blank(' ');
        let file = File::open(filename)?;
        let lines = std::io::BufReader::new(file).lines();
        for word in lines.flatten() {
            dict.add_word(&arena, &word);
        }
        Ok(dict)
    }
}

impl<'a> TrieLinkedListArena<'a> {
    fn add_word(&self, arena: &'a Arena<TrieLinkedListArena<'a>>, word: &str) {
        let mut cursor = self;
        let mut chars = word.chars().peekable();
        // println!("adding word {}", word);
        while let Some(c) = chars.next() {
            let is_last = chars.peek().is_none();
            match cursor.child.get() {
                None => {
                    // println!("none child");
                    let new = arena.alloc(TrieLinkedListArena {
                        child: Cell::new(None),
                        sibling: Cell::new(None),
                        letter: c,
                        is_word: is_last,
                    });
                    cursor.child.set(Some(new));
                    cursor = new;
                }
                Some(ref child) if child.letter > c => {
                    // println!("first child greater");
                    // First child here is greater than char to insert so we need to insert in first position and make parent.child point to new node
                    cursor.child.set(Some(arena.alloc(TrieLinkedListArena {
                        child: Cell::new(None),
                        sibling: Cell::new(cursor.child.take()),
                        letter: c,
                        is_word: is_last,
                    })));
                    cursor = cursor.child.get().as_mut().unwrap();
                }
                Some(ref mut child) => {
                    cursor = child;
                    while cursor.letter != c {
                        match cursor.sibling.get() {
                            None => {
                                // println!("none sibling");
                                cursor.sibling.set(Some(arena.alloc(TrieLinkedListArena {
                                    child: Cell::new(None),
                                    sibling: Cell::new(None),
                                    letter: c,
                                    is_word: is_last,
                                })));
                            }
                            Some(ref sibling) if sibling.letter > c => {
                                cursor.sibling.set(Some(arena.alloc(TrieLinkedListArena {
                                    child: Cell::new(None),
                                    sibling: Cell::new(cursor.sibling.take()),
                                    letter: c,
                                    is_word: is_last,
                                })));
                            }
                            _ => {}
                        };
                        cursor = cursor.sibling.get().as_mut().unwrap()
                    }
                }
            }
        }
        // cursor.is_word = true;
    }
}
impl Trie for TrieLinkedListArena<'_> {
    fn traverse(&self, word: &str) -> Option<&dyn Trie> {
        let mut cursor = self;
        for c in word.chars() {
            match cursor.child.get() {
                Some(child) => cursor = child,
                None => return None,
            }
            while cursor.letter != c {
                if cursor.letter > c {
                    return None;
                } else {
                    match cursor.sibling.get() {
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
        let arena = Arena::new();
        let mut dict = TrieLinkedListArena::blank(' ');
        dict.add_word(&arena, "hell");
        dict.add_word(&arena, "abc");
        dict.add_word(&arena, "hello");
        dict.add_word(&arena, "spoon");
        dict.add_word(&arena, "spoonlike");
        dict.add_word(&arena, "spoony");
        dict.add_word(&arena, "spoonmaker");
        dict.add_word(&arena, "spoonmaking");
        dict.add_word(&arena, "spoons");
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
        let arena = Arena::new();
        TrieLinkedListArena::from_file("./words_alpha.txt", &arena).unwrap();
        assert_eq!(arena.len(), 1027814)
        // while stack.len() > 0 {
        //     let dict = stack.pop().unwrap();
        //     count += 1;
        //     stack.append(&mut dict.child.values().collect())
        // }
        // assert_eq!(count, 1027815);
    }

    #[bench]
    fn bench_build_dict(b: &mut Bencher) {
        b.iter(|| {
            TrieLinkedListArena::from_file("./words_alpha.txt", &Arena::with_capacity(1027814))
                .unwrap();
            ' '
        })
    }
}
