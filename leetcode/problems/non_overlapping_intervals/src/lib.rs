pub struct Solution;

impl Solution {
    // [[1,2],[2,3],[3,4],[1,3]]
    //
    // 1  2 3   4
    // █  █ █   █
    // █▀▀█▀█▀▀▀▀
    // █  ▀▀█
    // ▀▀▀▀▀▀
    //
    // Thinking process:
    // - when overlapping, remove the interval with larger right boundary
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() < 2 {
            return 0;
        }
        intervals.sort_unstable_by_key(|v| v[0]);
        // println!("intervals={intervals:?}");
        let mut count = 0;
        let mut x = &intervals[0];
        // println!("x={x:?}");
        for y in &intervals[1..] {
            // overlapping
            if y[0] >= x[0] && y[0] < x[1] {
                count += 1;
                if x[1] >= y[1] {
                    x = y;
                }
            } else {
                x = y;
            }
            // println!("x={x:?} count={count}");
        }
        count
    }
}
