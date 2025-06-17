pub struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        Solution::longest_increasing_path_with_mem(matrix)
    }

    pub fn longest_increasing_path_with_mem(matrix: Vec<Vec<i32>>) -> i32 {
        /// return the maximum depth from (row, col) of matrix
        fn dfs(
            matrix: &[&[i32]],
            row: usize,
            col: usize,
            seen: &mut Vec<Vec<bool>>,
            mem: &mut Vec<Vec<usize>>,
        ) -> usize {
            let dirs = [[0, -1], [-1, 0], [0, 1], [1, 0]];
            let mut local_max_depth = 1;
            for dir in dirs {
                let next_row = row as isize + dir[0];
                let next_col = col as isize + dir[1];

                // boundary testing
                if next_row < 0
                    || next_row >= matrix.len() as isize
                    || next_col < 0
                    || next_col >= matrix[0].len() as isize
                {
                    continue;
                }

                let next_row = next_row as usize;
                let next_col = next_col as usize;

                // visibility testing
                if seen[next_row][next_col] {
                    continue;
                }
                seen[next_row][next_col] = true;

                if matrix[next_row][next_col] > matrix[row][col] {
                    let local_detph = if mem[next_row][next_col] != 0 {
                        1 + mem[next_row][next_col]
                    } else {
                        1 + dfs(matrix, next_row, next_col, seen, mem)
                    };

                    // println!("row={row} col={col} next_row={next_row} next_col={next_col} local_detph={local_detph}");
                    local_max_depth = local_max_depth.max(local_detph);
                }
                seen[next_row][next_col] = false; // backtrack
            }
            mem[row][col] = local_max_depth;
            // println!("mem=");
            // for v in mem {
            //     println!("{v:?}");
            // }
            local_max_depth
        }
        let m: Vec<&[i32]> = matrix.iter().map(|row| row.as_slice()).collect();
        let mut ans = 1;
        let mut mem = vec![vec![0usize; matrix[0].len()]; matrix.len()];
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                let mut seen = vec![vec![false; matrix[0].len()]; matrix.len()];
                ans = ans.max(dfs(&m, row, col, &mut seen, &mut mem));
            }
        }
        ans as i32
    }

    // Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
    // 9,9,4
    // 6,6,8
    // 2,1,1
    //
    // First thought, use BFS to explore every path. O((m*n)^2)
    //
    // (0,0) -> (0,1) ❌
    // (0,0) -> (1,0) ❌
    //
    // (0,1) -> (0,0) ❌
    // (0,1) -> (0,2) ❌
    // (0,1) -> (1,1) ❌
    //
    // (0,2) -> (0,1) ✅
    // (0,2) -> (1,2) ✅
    //
    // (1,0) -> (0,0) ✅
    // (1,0) -> (1,1) ❌
    // (1,0) -> (2,0) ❌
    //
    // For any position, (r,c),
    // if M(r,c-1) > M(r,c), then f(r,c) = 1 + f(r,c-1) // left
    // if M(r-1,c) > M(r,c), then f(r,c) = 1 + f(r-1,c) // top
    // if M(r,c+1) > M(r,c), then f(r,c) = 1 + f(r,c+1) // right
    // if M(r+1,c) > M(r,c), then f(r,c) = 1 + f(r+1,c) // bottom
    //
    // 1,2,3
    // 6,5,4
    // 7,8,9
    //
    // 1 ┌─> 2 ┌─> 3 ──> 4 ┌─> 5 ┌─> 6 ──> 7 ──> 8 ──> 9
    //   │     │           │     └─> 8 ──> 9
    //   │     │           └─> 9
    //   │     └─> 5 ┌─> 6 ──> 7 ──> 8 ──> 9
    //   │           └─> 8 ──> 9
    //   └─> 6 ──> 7 ──> 8 ──> 9
    //
    // However, from the above tree graph, we could know DFS should be used
    // and with memoization we can avoid the subproblems recomputing overhead.
    //
    // Time Limit Exceeded version
    pub fn longest_increasing_path_dfs(matrix: Vec<Vec<i32>>) -> i32 {
        fn dfs(
            matrix: &[&[i32]],
            row: usize,
            col: usize,
            depth: usize,
            max_depth: &mut usize,
            seen: &mut Vec<Vec<bool>>,
        ) -> usize {
            let dirs = [[0, -1], [-1, 0], [0, 1], [1, 0]];
            for dir in dirs {
                let next_row = row as isize + dir[0];
                let next_col = col as isize + dir[1];

                // boundary testing
                if next_row < 0
                    || next_row >= matrix.len() as isize
                    || next_col < 0
                    || next_col >= matrix[0].len() as isize
                {
                    continue;
                }

                let next_row = next_row as usize;
                let next_col = next_col as usize;

                // visibility testing
                if seen[next_row][next_col] {
                    continue;
                }
                seen[next_row][next_col] = true;

                if matrix[next_row][next_col] > matrix[row][col] {
                    let d = dfs(matrix, next_row, next_col, depth + 1, max_depth, seen);
                    *max_depth = (*max_depth).max(d);
                }
                seen[next_row][next_col] = false; // backtrack
            }
            depth
        }
        let m: Vec<&[i32]> = matrix.iter().map(|row| row.as_slice()).collect();
        let mut ans = 1;
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                let mut max_depth = 1;
                let mut seen = vec![vec![false; matrix[0].len()]; matrix.len()];
                dfs(&m, row, col, 1, &mut max_depth, &mut seen);
                // dfs(&m, 0, 0, 1, &mut max_depth, &mut seen); // for debug
                ans = ans.max(max_depth);
            }
        }
        ans as i32
    }
}
