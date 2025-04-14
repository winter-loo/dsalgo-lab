pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        // println!("rows={rows} cols={cols} target={target}");
        let (mut l, mut r) = (0, rows * cols);
        while l < r {
            let m = (l + r) / 2;
            let (x, y) = to_cell_index(m, cols);
            // println!("m={m} x={x} y={y}");
            if matrix[y][x] < target {
                l = m + 1;
            } else if matrix[y][x] > target {
                r = m;
            } else {
                return true;
            }
        }

        false
    }
}

/// return (x, y)
pub fn to_cell_index(i: usize, cols: usize) -> (usize, usize) {
    (i % cols, i / cols)
}
