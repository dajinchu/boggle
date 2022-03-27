#![feature(test)]
mod solver;
mod word_dict;

use std::fs::File;
use std::io::{self, BufRead};

use typed_arena::Arena;
use word_dict::linkedlist_typedarena::TrieLinkedListArena;

use crate::solver::find_best;
use crate::word_dict::Trie;
use crate::word_dict::hashmap::TrieHashMap;
use crate::word_dict::linkedlist::TrieLinkedList;
extern crate test;

type Board = Vec<Vec<char>>;
// Row, Col format
type Pos = (usize, usize);

fn main() {
    let arena = Arena::with_capacity(1027814);
    let words = TrieLinkedListArena::from_file("./words_alpha.txt", &arena).unwrap();

    let mut board: Board = Vec::new();
    let lines = io::BufReader::new(File::open("./board2.txt").unwrap()).lines();
    for row in lines.flatten() {
        board.push(row.chars().collect::<Vec<char>>());
    }

    // println!("{}", size_of::<[Box<word_dict::DictEntry>;26]>());

    // let stdin = io::stdin();
    // let mut d: &dyn Trie = &words;
    // for line in stdin.lock().lines() {
    //     d = d.traverse(line.unwrap().as_str()).unwrap();
    //     println!("{:?}", d);
    // }

    println!("{:?}", find_best(&words, &board));

    // println!("{:?}", words.valid_word_start("oranguta".to_string()));
}
