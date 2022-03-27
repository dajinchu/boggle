pub mod hashmap;
pub mod linkedlist;
pub mod vec;

pub trait Trie {
    fn add_word(&mut self, word: &str);
    fn traverse(&self, word: &str) -> Option<&dyn Trie>;
    fn is_word(&self) -> bool;
}
