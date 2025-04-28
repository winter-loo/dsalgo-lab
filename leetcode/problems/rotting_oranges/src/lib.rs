pub struct Solution;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        fn bfs(grid: &mut [Vec<i32>], myq: &mut VecDeque<(usize, usize)>, count_mins: &mut u32) {
            while !myq.is_empty() {
                let qlen = myq.len();
                *count_mins += 1;
                for _ in 0..qlen {
                    if let Some((row, col)) = myq.pop_front() {
                        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                        for (dr, dc) in dirs {
                            let (next_row, next_col) = (row as isize + dr, col as isize + dc);
                            if next_row < 0
                                || next_col < 0
                                || next_row as usize >= grid.len()
                                || next_col as usize >= grid[0].len()
                            {
                                continue;
                            }
                            let (next_row, next_col) = (next_row as usize, next_col as usize);
                            // visited or empty
                            if grid[next_row][next_col] == 6
                                || grid[next_row][next_col] == 0
                                || grid[next_row][next_col] == 2
                            {
                                continue;
                            }

                            assert_eq!(grid[next_row][next_col], 1);

                            grid[next_row][next_col] = 6;

                            myq.push_back((next_row, next_col));
                        }
                    }
                }
            }
            // the first time being pushed into queue should not be counted
            *count_mins = count_mins.saturating_sub(1);
        }
        assert!(!grid.is_empty() && !grid[0].is_empty());
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut count_mins = 0;
        let mut myq = VecDeque::new();
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == 2 {
                    // mark the cell visited
                    grid[row][col] = 6;
                    myq.push_back((row, col));
                }
            }
        }
        bfs(&mut grid, &mut myq, &mut count_mins);
        if grid.iter().any(|v| v.contains(&1)) {
            -1
        } else {
            count_mins as i32
        }
    }
}
