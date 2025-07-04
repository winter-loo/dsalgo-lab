pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|v| v[0]); 
        if intervals.len() < 2 {
            return intervals;
        }
        let mut answer = vec![];
        let mut merging = &intervals[0];
        let mut merged: Vec<i32>;
        for intr in intervals[1..].iter() {
            if merging[0] > intr[1] || intr[0] > merging[1] {
                // println!("none overlapping");
                if intr[0] > merging[1] {
                    answer.push(merging.clone());
                    merging = intr;
                }
            } else {
                // println!("overlapping");
                merged = vec![intr[0].min(merging[0]), intr[1].max(merging[1])];
                merging = &merged;
            }
        }
        answer.push(merging.clone());
        answer
    }
}
