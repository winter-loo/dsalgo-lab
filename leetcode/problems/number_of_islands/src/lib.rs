pub struct Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        assert!(!grid.is_empty() && !grid[0].is_empty());
        let (rows, cols) = (grid.len(), grid[0].len());

        fn dfs(grid: &mut [Vec<char>], pos: (isize, isize)) {
            let (row, col) = (pos.0 as usize, pos.1 as usize);
            if pos.0 < 0 || pos.1 < 0 || row >= grid.len() || col >= grid[0].len() {
                return;
            }
            if grid[row][col] == 'V' {
                return;
            }

            if grid[row][col] == '0' {
                return;
            }

            // mark current cell counted
            grid[row][col] = 'V';

            let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dr, dc) in dirs {
                dfs(grid, (pos.0 + dr, pos.1 + dc));
            }
        }

        let mut count = 0;
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == '1' {
                    dfs(&mut grid, (row as isize, col as isize));
                    count += 1;
                }
            }
        }
        count
    }
}
