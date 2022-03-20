#![feature(test)]
mod solver;
mod word_dict;

use std::fs::File;
use std::mem::size_of;
use std::io::{self, BufRead};

use crate::solver::find_best;
use crate::word_dict::DictEntry;
extern crate test;

type Board = Vec<Vec<char>>;
// Row, Col format
type Pos = (usize, usize);

fn main() {
    let words = DictEntry::from_file("./words_alpha.txt").unwrap();

    let mut board: Board = Vec::new();
    let lines = io::BufReader::new(File::open("./board1.txt").unwrap()).lines();
    for row in lines.flatten() {
        board.push(row.chars().collect::<Vec<char>>());
    }

    println!("{}", size_of::<[Box<word_dict::DictEntry>;26]>());

    // println!("{:}", find_best(&words, &board));

    // println!("{:?}", words.valid_word_start("oranguta".to_string()));
}
