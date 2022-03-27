pub mod hashmap;
pub mod linkedlist;
pub mod linkedlist_typedarena;
pub mod vec;
pub mod naive;

pub trait Trie: std::fmt::Debug {
    // fn add_word(&mut self, word: &str);
    fn traverse(&self, word: &str) -> Option<&dyn Trie>;
    fn is_word(&self) -> bool;
}
