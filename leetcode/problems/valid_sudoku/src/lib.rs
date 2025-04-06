pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    matches!(myimpl(board), Some(true))
}

use std::collections::HashSet;

pub fn myimpl(board: Vec<Vec<char>>) -> Option<bool> {
    // all rows
    for y in 0..9 {
        if !is_valid_box(board.get(y)?.iter()) {
            return Some(false);
        }
    }

    // all columns
    for x in 0..9 {
        if !is_valid_box((0..9).filter_map(|y| board.get(y)?.get(x))) {
            return Some(false);
        }
    }

    // sub-boxes
    for y in (0..9).step_by(3) {
        if !board
            .get(y)?
            .chunks_exact(3) // [], [], []
            .zip(board.get(y + 1)?.chunks_exact(3)) // ([], []), ([], [])
            .zip(board.get(y + 2)?.chunks_exact(3)) // (([], []), [])
            .all(|((chk1, chk2), chk3)| {
                is_valid_box(chk1.iter().chain(chk2.iter()).chain(chk3.iter()))
            })
        {
            return Some(false);
        }
    }

    Some(true)
}

fn is_valid_box<'a>(iter: impl Iterator<Item = &'a char>) -> bool {
    let mut seen = HashSet::new();
    iter.filter(|cell| **cell != '.')
        .all(|cell| seen.insert(cell))
}
