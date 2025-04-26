pub struct Solution;

impl Solution {
    pub fn exist(board: &Vec<Vec<char>>, word: String) -> bool {
        use std::collections::HashSet;

        fn dfs(
            board: &Vec<Vec<char>>,
            word: &[u8],
            pos: (isize, isize),
            index: usize,
            visited: &mut HashSet<(isize, isize)>,
        ) -> bool {
            if pos.0 < 0
                || pos.1 < 0
                || pos.0 >= board[0].len() as isize
                || pos.1 >= board.len() as isize
            {
                return false;
            }
            if word[index] != board[pos.1 as usize][pos.0 as usize] as u8 {
                return false;
            }
            if index == word.len() - 1 {
                return true;
            }
            let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            for dir in directions {
                let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                if visited.contains(&next_pos) {
                    continue;
                }
                visited.insert(next_pos);
                if dfs(board, word, next_pos, index + 1, visited) {
                    return true;
                }
                visited.remove(&next_pos);
            }
            false
        }
        let (width, height) = (board[0].len(), board.len());

        if width == 1 && height == 1 {
            return word.len() == 1 && board[0][0] as u8 == word.as_bytes()[0];
        }

        if width * height < word.len() {
            return false;
        }

        let mut alphabet = HashSet::new();
        for y in 0..height {
            for x in 0..width {
                alphabet.insert(board[y][x]);
            }
        }
        if word.chars().any(|b| !alphabet.contains(&b)) {
            return false;
        }

        for y in 0..height {
            for x in 0..width {
                let mut visited = HashSet::new();
                visited.insert((x as isize, y as isize));
                if dfs(
                    &board,
                    word.as_bytes(),
                    (x as isize, y as isize),
                    0,
                    &mut visited,
                ) {
                    return true;
                }
                visited.remove(&(x as isize, y as isize));
            }
        }
        false
    }

    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        for word in words {
            if Solution::exist(&board, word.clone()) {
                result.push(word);
            }
        }
        result
    }
}
