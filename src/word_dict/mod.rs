pub mod hashmap;
pub mod linkedlist;
pub mod linkedlist_typedarena;
pub mod vec;
pub mod naive;
pub mod radix_lib;
pub mod yada;

pub trait Trie {
    // fn add_word(&mut self, word: &str);
    fn traverse(&self, word: &str) -> Option<&Self>;
    fn is_word(&self) -> bool;
}
