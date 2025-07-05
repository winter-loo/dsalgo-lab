pub struct Solution;

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        if intervals.len() < 2 {
            return true;
        }
        let mut x = &intervals[0];
        for y in &intervals[1..] {
            if y[0] >= x[0] && y[0] < x[1] {
                return false;
            }
            x = y;
        }
        true
    }
}
