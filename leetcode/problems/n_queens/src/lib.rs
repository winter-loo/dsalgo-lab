pub struct Solution;

impl Solution {
    // KEY INSIGHTS: we need backtrack row by row.
    // For each row, we need validate each column from the beginning to the end
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        #[allow(unused)]
        fn print_board(rows: &[Vec<char>]) {
            for row in rows {
                for x in 0..row.len() {
                    print!("{}", row[x]);
                    if x + 1 != row.len() {
                        print!(" ");
                    }
                }
                println!();
            }
        }
        fn is_valid_row(rows: &[Vec<char>], n: usize, pos: (usize, usize)) -> bool {
            if rows.is_empty() {
                return true;
            }

            // must not in the same column
            if rows.iter().any(|x| x[pos.0] == 'Q') {
                return false;
            }

            // must not in the diagonal
            let mut p = (pos.0 as isize, pos.1 as isize);
            while p.0 > 0 && p.1 > 0 {
                p.0 -= 1;
                p.1 -= 1;
                if rows[p.1 as usize][p.0 as usize] == 'Q' {
                    return false;
                }
            }

            // must not in the anti-diagonal
            let mut p = (pos.0, pos.1 as isize);
            while p.0 < n - 1 && p.1 > 0 {
                p.0 += 1;
                p.1 -= 1;
                if rows[p.1 as usize][p.0] == 'Q' {
                    return false;
                }
            }

            true
        }

        fn backtrack_row(
            index: usize,
            n: usize,
            rows: &mut Vec<Vec<char>>,
            results: &mut Vec<Vec<String>>,
        ) -> bool {
            // println!("index={index:?}");
            // print_board(rows);

            if index == n {
                results.push(
                    rows.clone()
                        .into_iter()
                        .map(|v| v.into_iter().collect())
                        .collect(),
                );
                return true;
            }

            for j in 0..n {
                let mut row = vec!['.'; n];
                row[j] = 'Q';
                if !is_valid_row(rows, n, (j, index)) {
                    continue;
                }
                rows.push(row);
                backtrack_row(index + 1, n, rows, results);
                rows.pop();
            }
            true
        }

        let mut rows = vec![];
        let mut results = vec![];
        backtrack_row(0, n as usize, &mut rows, &mut results);

        results
    }
}
