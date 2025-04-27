use std::collections::HashSet;

pub struct Solution;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Leftward,
    Rightward,
    Upward,
    Downward,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    direction: Direction,
    point: (usize, usize),
}

#[derive(Debug, Clone, Copy)]
struct MoveState {
    pos: Position,
    bound: (usize, usize),
}

impl MoveState {
    fn build(point: (usize, usize), bound: (usize, usize)) -> Vec<Self> {
        let mut directions = vec![];
        if point.0 >= 1 {
            directions.push(Direction::Leftward);
        }
        if point.0 + 1 < bound.0 {
            directions.push(Direction::Rightward);
        }
        if point.1 >= 1 {
            directions.push(Direction::Upward);
        }
        if point.1 + 1 < bound.1 {
            directions.push(Direction::Downward);
        }

        directions
            .into_iter()
            .map(|d| MoveState {
                pos: Position {
                    direction: d,
                    point,
                },
                bound,
            })
            .collect()
    }
}

impl MoveState {
    fn next_states(self) -> Vec<MoveState> {
        match self.pos.direction {
            Direction::Leftward => {
                let mut next_states = vec![];
                if self.pos.point.0 >= 1 {
                    next_states.push(MoveState {
                        pos: Position {
                            direction: Direction::Leftward,
                            point: (self.pos.point.0 - 1, self.pos.point.1),
                        },
                        bound: self.bound,
                    });
                }
                if self.pos.point.1 >= 1 {
                    next_states.push(MoveState {
                        pos: Position {
                            direction: Direction::Upward,
                            point: (self.pos.point.0, self.pos.point.1 - 1),
                        },
                        bound: self.bound,
                    });
                }
                if self.pos.point.1 + 1 < self.bound.1 {
                    next_states.push(MoveState {
                        pos: Position {
                            direction: Direction::Downward,
                            point: (self.pos.point.0, self.pos.point.1 + 1),
                        },
                        bound: self.bound,
                    });
                }
                next_states
            }
            Direction::Rightward => {
                let mut next_states = vec![];
                if self.pos.point.0 + 1 < self.bound.0 {
                    next_states.push(MoveState {
                        pos: Position {
                            direction: Direction::Rightward,
                            point: (self.pos.point.0 + 1, self.pos.point.1),
                        },
                        bound: self.bound,
                    });
                }
                if self.pos.point.1 >= 1 {
                    next_states.push(MoveState {
                        pos: Position {
                            direction: Direction::Upward,
                            point: (self.pos.point.0, self.pos.point.1 - 1),
                        },
                        bound: self.bound,
                    });
                }
                if self.pos.point.1 + 1 < self.bound.1 {
                    next_states.push(MoveState {
                        pos: Position {
                            direction: Direction::Downward,
                            point: (self.pos.point.0, self.pos.point.1 + 1),
                        },
                        bound: self.bound,
                    });
                }
                next_states
            }
            Direction::Upward => {
                let mut next_states = vec![];
                if self.pos.point.0 >= 1 {
                    next_states.push(MoveState {
                        pos: Position {
                            direction: Direction::Leftward,
                            point: (self.pos.point.0 - 1, self.pos.point.1),
                        },
                        bound: self.bound,
                    });
                }
                if self.pos.point.0 + 1 < self.bound.0 {
                    next_states.push(MoveState {
                        pos: Position {
                            direction: Direction::Rightward,
                            point: (self.pos.point.0 + 1, self.pos.point.1),
                        },
                        bound: self.bound,
                    });
                }
                if self.pos.point.1 >= 1 {
                    next_states.push(MoveState {
                        pos: Position {
                            direction: Direction::Upward,
                            point: (self.pos.point.0, self.pos.point.1 - 1),
                        },
                        bound: self.bound,
                    });
                }
                next_states
            }
            Direction::Downward => {
                let mut next_states = vec![];
                if self.pos.point.0 >= 1 {
                    next_states.push(MoveState {
                        pos: Position {
                            direction: Direction::Leftward,
                            point: (self.pos.point.0 - 1, self.pos.point.1),
                        },
                        bound: self.bound,
                    });
                }
                if self.pos.point.0 + 1 < self.bound.0 {
                    next_states.push(MoveState {
                        pos: Position {
                            direction: Direction::Rightward,
                            point: (self.pos.point.0 + 1, self.pos.point.1),
                        },
                        bound: self.bound,
                    });
                }
                if self.pos.point.1 + 1 < self.bound.1 {
                    next_states.push(MoveState {
                        pos: Position {
                            direction: Direction::Downward,
                            point: (self.pos.point.0, self.pos.point.1 + 1),
                        },
                        bound: self.bound,
                    });
                }
                next_states
            }
        }
    }
}
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() || board[0].is_empty() {
            return false;
        }
        
        let rows = board.len();
        let cols = board[0].len();
        let word_bytes = word.as_bytes();
        
        // Early termination checks
        if rows * cols < word.len() {
            return false;
        }
        
        if rows == 1 && cols == 1 {
            return word.len() == 1 && board[0][0] as u8 == word_bytes[0];
        }
        
        // Check if all characters in the word exist in the board
        let mut char_frequency = std::collections::HashMap::new();
        for row in &board {
            for &ch in row {
                *char_frequency.entry(ch).or_insert(0) += 1;
            }
        }
        
        for &byte in word_bytes {
            let ch = byte as char;
            let count = char_frequency.get(&ch).copied().unwrap_or(0);
            if count == 0 {
                return false;
            }
            // For repeated characters in the word, ensure we have enough of them in the board
            *char_frequency.entry(ch).or_insert(0) -= 1;
        }
        
        // Use a mutable 2D grid instead of HashSet for visited tracking (more efficient)
        let mut visited = vec![vec![false; cols]; rows];
        
        // Try starting from each cell
        for i in 0..rows {
            for j in 0..cols {
                if dfs(&board, i, j, 0, word_bytes, &mut visited) {
                    return true;
                }
            }
        }
        
        false
    }
}

// Helper DFS function with more efficient parameter types
fn dfs(
    board: &[Vec<char>],
    row: usize,
    col: usize,
    index: usize,
    word: &[u8],
    visited: &mut [Vec<bool>],
) -> bool {
    // If we've matched all characters in the word
    if index == word.len() {
        return true;
    }
    
    // Check bounds and character match
    if row >= board.len() || col >= board[0].len() || 
       visited[row][col] || board[row][col] as u8 != word[index] {
        return false;
    }
    
    // Mark as visited
    visited[row][col] = true;
    
    // Check if this is the last character we need to match
    if index == word.len() - 1 {
        return true;
    }
    
    // Try all four directions
    let directions = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)]; // down, right, up, left
    
    for &(dr, dc) in &directions {
        let (new_row, new_col) = match (dr, dc) {
            (0, usize::MAX) => { // up
                if row == 0 { continue; }
                (row - 1, col)
            },
            (usize::MAX, 0) => { // left
                if col == 0 { continue; }
                (row, col - 1)
            },
            _ => (row + dr, col + dc) // down or right
        };
        
        // Skip if out of bounds
        if new_row >= board.len() || new_col >= board[0].len() {
            continue;
        }
        
        if dfs(board, new_row, new_col, index + 1, word, visited) {
            return true;
        }
    }
    
    // Backtrack
    visited[row][col] = false;
    
    false
}
