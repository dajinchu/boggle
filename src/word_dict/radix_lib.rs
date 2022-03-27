use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, Error};

use super::Trie;

#[derive(Debug)]
pub struct TrieRadix {
    trie: radix_trie::Trie<String, bool>,
}

impl TrieRadix {
    pub fn blank(c: char) -> TrieRadix {
        TrieRadix {
            trie: radix_trie::Trie::new(),
        }
    }

    pub fn from_file(filename: &str) -> Result<TrieRadix, Error> {
        let mut dict = TrieRadix::blank(' ');
        let file = File::open(filename)?;
        let lines = std::io::BufReader::new(file).lines();
        for word in lines.flatten() {
            dict.add_word(&word);
        }
        Ok(dict)
    }
    fn add_word(&mut self, word: &str) {
        self.trie.insert(word.to_string(), true);
    }
}

// impl<'a> Trie for TrieRadix<'a> {
//     fn traverse(&self, word: &str) -> Option<&TrieRadix<'a>> {
//         match self.trie.get(word) {
//             None => None,
//             Some(trie) => Some(&TrieRadix { trie }),
//         }

//         //.and_then(|t| Some(TrieRadix { trie: t })).as_ref()
//     }

//     fn is_word(&self) -> bool {
//         *self.trie.get("").unwrap_or(&false)
//     }
// }
// impl Trie for SubTrieRadix<'_> {
//     fn traverse(&self, word: &str) -> Option<&dyn Trie> {
//         todo!()
//     }

//     fn is_word(&self) -> bool {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn simple_test() {
        let mut dict = TrieRadix::blank(' ');
        dict.add_word("hell");
        dict.add_word("abc");
        dict.add_word("hello");
        dict.add_word("spoon");
        dict.add_word("spoonlike");
        dict.add_word("spoony");
        dict.add_word("spoonmaker");
        dict.add_word("spoonmaking");
        dict.add_word("spoons");
        // assert_eq!(dict.traverse("abc").unwrap().is_word(), true);
        // assert_eq!(dict.traverse("hello").unwrap().is_word(), true);
        // assert_eq!(dict.traverse("he").unwrap().is_word(), false);
        // assert_eq!(dict.traverse("fjidso").is_none(), true);
        // assert_eq!(dict.traverse("spoonlike").unwrap().is_word(), true);
        // assert_eq!(dict.traverse("spoonmaker").unwrap().is_word(), true);
        // assert_eq!(dict.traverse("spoonmaking").unwrap().is_word(), true);
        // assert_eq!(dict.traverse("spoons").unwrap().is_word(), true);
        // assert_eq!(dict.traverse("spoonm").unwrap().is_word(), false);
    }

    #[test]
    fn dict_size() {
        let dict = TrieRadix::from_file("./words_alpha.txt").unwrap();
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
        b.iter(|| TrieRadix::from_file("./words_alpha.txt").unwrap())
    }
}
