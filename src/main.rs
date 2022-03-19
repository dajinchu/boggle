use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
struct DictEntry {
    next: Box<WordDict>,
    is_word: bool,
}

impl DictEntry {
    fn blank() -> DictEntry {
        DictEntry {
            next: Box::new([
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ]),
            is_word: false,
        }
    }

    fn traverse(&self, word: &str) -> Option<&DictEntry> {
        let mut cursor = self;
        for c in word.chars() {
            match cursor.next[char_to_index(c)].as_ref() {
                None => return None,
                Some(words) => {
                    cursor = words;
                }
            }
        }
        Some(cursor)
    }
}

type WordDict = [Option<DictEntry>; 26];

fn char_to_index(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

fn add_word(word: String, words: &mut DictEntry) {
    let mut cursor = words;
    for c in word.chars() {
        if cursor.next[char_to_index(c)].is_none() {
            cursor.next[char_to_index(c)] = Some(DictEntry::blank());
        }
        let entry = cursor.next.as_mut()[char_to_index(c)]
            .as_mut()
            .expect("failed to build word dict");
        cursor = entry;
    }
    cursor.is_word = true;
}

type Board = Vec<Vec<char>>;
// Row, Col format
type Pos = (usize, usize);

// Find longest word in the board
fn find_best(words: &DictEntry, board: &Board) -> String {
    let mut best = "".to_string();
    let height = board.len();
    let width = board[0].len();
    for row in 0..height {
        for col in 0..width {
            let word = find_best_acc(
                words,
                board,
                (row, col),
                &mut "".to_string(),
                &mut Vec::new(),
            );
            if word.chars().count() > best.chars().count() {
                best = word;
            }
        }
    }
    best
}

// Find longest word in the board if we've already taken the given path to build the given string
fn find_best_acc(
    words: &DictEntry,
    board: &Board,
    pos: Pos,
    word_so_far: &mut String,
    path: &mut Vec<Pos>,
) -> String {
    let char_at = board[pos.0][pos.1];
    if !char_at.is_alphabetic() {
        return "".to_string();
    }
    word_so_far.push(char_at);
    // println!("path: {:?}, word: {:?}", path, word_so_far);

    match words.traverse(word_so_far) {
        Some(dict) => {
            path.push(pos);
            let best = neighbors(board, pos)
                .iter()
                .filter(|p| !path.contains(*p))
                .map(|p| {
                    find_best_acc(
                        words,
                        board,
                        *p,
                        &mut word_so_far.clone(),
                        &mut path.clone(),
                    )
                })
                .max_by(|x, y| x.chars().count().cmp(&y.chars().count()));
            match best {
                Some(s) => s,
                None => {
                    if dict.is_word {
                        word_so_far.to_string()
                    } else {
                        "".to_string()
                    }
                }
            }
        }
        None => "".to_string(),
    }
}

// get the neighbors of this pos
fn neighbors(board: &Board, pos: Pos) -> Vec<Pos> {
    let width = board[0].len();
    let height = board.len();
    let mut v = Vec::with_capacity(8);

    let row = pos.0;
    let col = pos.1;
    if row > 0 {
        if col > 0 {
            v.push((row - 1, col - 1));
            v.push((row, col - 1));
        }
        if col + 1 < width {
            v.push((row - 1, col + 1));
            v.push((row, col + 1));
        }
        v.push((row - 1, col));
    }
    if row + 1 < height {
        if col > 0 {
            v.push((row + 1, col - 1));
        }
        if col + 1 < width {
            v.push((row + 1, col + 1));
        }
        v.push((row + 1, col));
    }
    v
}

fn main() {
    let mut words = DictEntry::blank();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./words_alpha.txt") {
        for word in lines.flatten() {
            add_word(word, &mut words);
        }
    }

    let mut board: Board = Vec::new();
    if let Ok(lines) = read_lines("./board1.txt") {
        for row in lines.flatten() {
            board.push(row.chars().collect::<Vec<char>>());
        }
    }

    println!("{:}", find_best(&words, &board));

    // println!("{:?}", words.valid_word_start("oranguta".to_string()));
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
