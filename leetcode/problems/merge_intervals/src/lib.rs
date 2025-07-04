pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() < 2 {
            return intervals;
        }
        let mut answer = vec![intervals[0].clone()];
        for intr in intervals[1..].into_iter() {
        }
        answer
    }
}
