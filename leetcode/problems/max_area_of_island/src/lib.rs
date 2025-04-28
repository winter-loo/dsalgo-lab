pub struct Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        assert!(!grid.is_empty() && !grid[0].is_empty());
        let (rows, cols) = (grid.len(), grid[0].len());

        fn dfs(grid: &mut [Vec<i32>], pos: (isize, isize), area: &mut i32) {
            let (row, col) = (pos.0 as usize, pos.1 as usize);
            if pos.0 < 0 || pos.1 < 0 || row >= grid.len() || col >= grid[0].len() {
                return;
            }
            if grid[row][col] == 6 {
                return;
            }

            if grid[row][col] == 0 {
                return;
            }

            // mark current cell visited
            grid[row][col] = 6;
            *area += 1;

            let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dr, dc) in dirs {
                dfs(grid, (pos.0 + dr, pos.1 + dc), area);
            }
        }

        let mut max_area = 0;
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == 1 {
                    let mut area = 0;
                    dfs(&mut grid, (row as isize, col as isize), &mut area);
                    max_area = max_area.max(area);
                }
            }
        }
        max_area
    }
}
