use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, Error};

use yada::builder::DoubleArrayBuilder;
use yada::DoubleArray;

use super::Trie;

pub struct TrieYada {
    trie: yada::DoubleArray<Vec<u8>>,
}

impl TrieYada {
    pub fn from_file(filename: &str) -> Result<TrieYada, Error> {
        let file = File::open(filename)?;
        let lines = std::io::BufReader::new(file).lines();
        Ok(TrieYada::from_words(lines.flatten().collect()))
    }
    fn from_words(words: Vec<String>) -> TrieYada {
        let mut vec = words.clone();
        vec.sort();
        let bytes = vec.into_iter().map(|w| (w.into_bytes(), 1)).collect::<Vec<(Vec<u8>, u32)>>();
        TrieYada {
            trie: DoubleArray::new(
                DoubleArrayBuilder::build(&bytes).unwrap(),
            ),
        }
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
        let mut dict = TrieYada::from_words(
            vec![
                "hell",
                "abc",
                "hello",
                "spoon",
                "spoonlike",
                "spoony",
                "spoonmaker",
                "spoonmaking",
                "spoons",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );
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
        let dict = TrieYada::from_file("./words_alpha.txt").unwrap();
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
        b.iter(|| TrieYada::from_file("./words_alpha.txt").unwrap())
    }
}
