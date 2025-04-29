pub struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn dfs(board: &mut Vec<Vec<char>>, pos: (isize, isize)) {
            // print_board(board);
            let (row, col) = (pos.0, pos.1);
            if row < 0 || col < 0 || row >= board.len() as isize || col >= board[0].len() as isize {
                return;
            }
            let (row, col) = (row as usize, col as usize);

            if board[row][col] == 'X' || board[row][col] == 'V' {
                return;
            }

            assert_eq!(board[row][col], 'O');

            board[row][col] = 'V';
            let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dr, dc) in dirs {
                dfs(board, (pos.0 + dr, pos.1 + dc));
            }
        }
        assert!(!board.is_empty() && !board[0].is_empty());
        let (rows, cols) = (board.len() as isize, board[0].len() as isize);

        for col in 0..cols {
            dfs(board, (0, col));
            dfs(board, (rows - 1, col));
        }

        for row in 0..rows {
            dfs(board, (row, 0));
            dfs(board, (row, cols - 1));
        }

        let (rows, cols) = (rows as usize, cols as usize);
        for row in 0..rows {
            for col in 0..cols {
                if board[row][col] == 'O' {
                    board[row][col] = 'X';
                } else if board[row][col] == 'V' {
                    board[row][col] = 'O';
                }
            }
        }
    }

    // XXX: This is my initial solution
    // Has a bug in it. See the example below.
    // if not explore from the border, you have to test whether the current 'O'
    // can be surrounded. And when backtracking, you found a bordered 'O'. You
    // lose the explored 'O' positions. So, you have to re-explore from the bordered
    // 'O'.
    //
    // ['X', 'X', 'X', 'X'],
    // ['X', 'O', 'O', 'X'],
    // ['X', 'O', 'O', 'X'],
    // ['X', 'O', 'X', 'X'],
    //
    // when you start exploring from (2, 1)
    pub fn solve_exploring_not_from_borders(board: &mut Vec<Vec<char>>) {
        fn dfs(board: &mut [Vec<char>], pos: (usize, usize)) -> bool {
            print_board(&board);
            let (row, col) = (pos.0, pos.1);
            if row == 0 || col == 0 || row == board.len() - 1 || col == board[0].len() - 1 {
                // the region can not be surrounded
                return board[row][col] == 'O';
            }

            if board[row][col] == 'X' || board[row][col] == 'V' {
                return false;
            }

            assert_eq!(board[row][col], 'O');

            board[row][col] = 'V';

            let mut cannot_be_surrounded = false;
            let dirs = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
            for (dr, dc) in dirs {
                let next_row = if dr < 0 {
                    row.saturating_sub((-dr) as usize)
                } else {
                    row + dr as usize
                };
                let next_col = if dc < 0 {
                    col.saturating_sub((-dc) as usize)
                } else {
                    col + dc as usize
                };
                if dfs(board, (next_row, next_col)) {
                    cannot_be_surrounded = true;
                    break;
                }
            }

            println!("cannot_be_surrounded={cannot_be_surrounded}");
            board[row][col] = if cannot_be_surrounded { 'O' } else { 'X' };

            cannot_be_surrounded
        }
        assert!(!board.is_empty() && !board[0].is_empty());
        let (rows, cols) = (board.len(), board[0].len());

        for row in 0..rows {
            for col in 0..cols {
                if board[row][col] == 'O' {
                    dfs(board, (row, col));
                }
            }
        }
    }
}

fn print_board(board: &[Vec<char>]) {
    for row in board {
        println!("{row:?}");
    }
    println!("------------");
}
