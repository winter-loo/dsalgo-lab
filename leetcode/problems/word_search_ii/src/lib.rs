use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        assert!(!board.is_empty() && !board[0].is_empty());

        let mut word_trie = Trie::new();
        for word in words {
            word_trie.insert(word);
        }
        let (rows, cols) = (board.len(), board[0].len());
        let mut path = vec![];
        let mut result = vec![];

        for row in 0..rows {
            for col in 0..cols {
                Solution::dfs(
                    &mut board,
                    (row as isize, col as isize),
                    &mut word_trie.root,
                    &mut path,
                    &mut result,
                );
            }
        }

        result
    }

    fn dfs(
        board: &mut [Vec<char>],
        pos: (isize, isize),
        node: &mut TrieNode,
        path: &mut Vec<char>,
        result: &mut Vec<String>,
    ) {
        // println!("path={path:?}");
        if pos.0 < 0 || pos.1 < 0 {
            return;
        }
        let (row, col) = (pos.0 as usize, pos.1 as usize);
        if row >= board.len() || col >= board[0].len() {
            return;
        }
        // already visited?
        if board[row][col] == 'V' {
            return;
        }

        // invalid path?
        let Some(next_node) = node.children.get_mut(&board[row][col]) else {
            return;
        };

        // mark currrent cell visited
        // println!("mark visited: {row}:{col}");
        let chr = board[row][col];
        board[row][col] = 'V';
        path.push(chr);

        // println!("end of word: {}", next_node.is_end_of_word);
        if next_node.is_end_of_word {
            let word: String = path.iter().collect();
            next_node.is_end_of_word = false; // de-duplicate
            result.push(word);
        }

        let delta_moves = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
        for (dr, dc) in delta_moves {
            Solution::dfs(
                board,
                (row as isize + dr, col as isize + dc),
                next_node,
                path,
                result,
            );
        }

        // println!("unmark visited: {row}:{col}");
        board[row][col] = chr;
        path.pop();
    }

    // Time Limit Exceeded
    pub fn find_words_one_by_one(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        for word in words {
            if Solution::exist(&board, word.clone()) {
                result.push(word);
            }
        }
        result
    }

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
}

#[derive(Debug)]
pub struct TrieNode {
    is_end_of_word: bool,
    children: HashMap<char, TrieNode>,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode {
                is_end_of_word: false,
                children: HashMap::new(),
            },
        }
    }

    pub fn insert(&mut self, word: String) {
        let len = word.len();
        let mut node = &mut self.root;
        for (i, c) in word.chars().into_iter().enumerate() {
            node = node.children.entry(c).or_insert(TrieNode {
                is_end_of_word: i + 1 == len,
                children: HashMap::new(),
            });
            if i + 1 == len {
                node.is_end_of_word = true;
            }
        }
    }

    pub fn search(&self, word: String) -> bool {
        let len = word.len();
        let mut node = &self.root;
        for (i, c) in word.chars().enumerate() {
            if let Some(x) = node.children.get(&c) {
                node = x;
                if i + 1 == len {
                    return x.is_end_of_word;
                }
            } else {
                return false;
            }
        }
        false
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            if let Some(x) = node.children.get(&c) {
                node = x;
            } else {
                return false;
            }
        }
        true
    }
}
