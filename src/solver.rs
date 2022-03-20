use crate::{word_dict::DictEntry, Board, Pos};

// Find longest word in the board
pub fn find_best(words: &DictEntry, board: &Board) -> String {
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
            if let Some(w) = word {
                if w.chars().count() > best.chars().count() {
                    best = w;
                }
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
) -> Option<String> {
    let char_at = board[pos.0][pos.1];
    if !char_at.is_alphabetic() {
        return None;
    }
    word_so_far.push(char_at);
    // println!("path: {:?}, word: {:?}", path, word_so_far);

    match words.traverse(word_so_far) {
        Some(dict) => {
            path.push(pos);
            let best = neighbors(board, pos)
                .iter()
                .filter(|p| !path.contains(*p))
                .filter_map(|p| {
                    find_best_acc(
                        words,
                        board,
                        *p,
                        &mut word_so_far.clone(),
                        &mut path.clone(),
                    )
                })
                .max_by(|x, y| x.chars().count().cmp(&y.chars().count()));
            // println!("word so far: {:} best: {:?}", word_so_far, best);
            match best {
                Some(s) => Some(s),
                None => {
                    if dict.is_word {
                        Some(word_so_far.to_string())
                    } else {
                        None
                    }
                }
            }
        }
        None => None,
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

#[cfg(test)]
mod tests {
    use crate::word_dict::DictEntry;

    use test::Bencher;
    use super::*;

    fn make_dict() -> DictEntry {
        DictEntry::from_file("./words_alpha.txt").unwrap()
    }

    #[test]
    fn it_works() {
        let words = make_dict();
        let board = vec![
            vec!['x', 'y', 'q'],
            vec!['h', ' ', 'o'],
            vec!['e', 'l', 'l'],
        ];
        assert_eq!(find_best(&words, &board), "hello");
    }

    #[bench]
    fn bench_solver(b: &mut Bencher) {
        let words = make_dict();
        let board = vec![
            vec!['x', 'y', 'q'],
            vec!['h', ' ', 'o'],
            vec!['e', 'l', 'l'],
        ];
        b.iter(||find_best(&words, &board));
    }
}
