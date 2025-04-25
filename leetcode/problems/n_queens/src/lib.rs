pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn solve(
            pos: (usize, usize),
            n: usize,
            rows: &mut Vec<Vec<char>>,
            results: &mut Vec<Vec<String>>,
        ) {
            println!("pos={pos:?}, rows={rows:#?}");
            if pos.0 >= n || pos.1 >= n {
                return;
            }

            if pos.0 == n {
                results.push(rows.clone().into_iter().map(|v| v.into_iter().collect()).collect());
                return;
            }

            // // must not in the same row
            // if rows[pos.1].iter().any(|col| *col == b'Q') {
            //     return;
            // }

            // // must not in the same column
            // if rows.iter().any(|row| row[pos.0] == b'Q') {
            //     return;
            // }

            // must not in the diagonal
            let mut p = (pos.0 as isize, pos.1 as isize);
            while p.0 > 0 && p.1 > 0 {
                p.0 -= 1;
                p.1 -= 1;
                if rows[p.1 as usize][p.0 as usize] == 'Q' {
                    return;
                }
            }

            // must not in the anti-diagonal
            let mut p = (pos.0, pos.1 as isize);
            while p.0 <= n && p.1 > 0 {
                p.0 += 1;
                p.1 -= 1;
                if rows[p.1 as usize][p.0 as usize] == 'Q' {
                    return;
                }
            }

            for i in pos.0..n {
                if i + 1 < rows.len() {
                    rows.push(vec!['.'; n]);
                }
                let mut row = ;
                row[index.0][index.1] = 'Q';
                rows.push(row);
                solve(i + 1, rows, results);
            }
        }

        let mut rows = vec![];
        let mut results = vec![];
        solve((0, 0), n as usize, &mut rows, &mut results);

        results
    }
}
