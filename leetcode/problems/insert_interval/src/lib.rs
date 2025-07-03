pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval];
        }
        let mut answer = vec![];
        let mut inserted = false;
        for intrv in intervals {
            if inserted {
                answer.push(intrv);
                continue;
            }
            if intrv[0] > new_interval[1] || new_interval[0] > intrv[1] {
                // println!("non overlapping");
                if new_interval[0] < intrv[0] {
                    answer.push(new_interval.clone());
                    inserted = true;
                }
                answer.push(intrv);
            } else {
                // println!("overlapping");
                // merge intervals
                new_interval = vec![intrv[0].min(new_interval[0]), intrv[1].max(new_interval[1])];
            }
        }
        if !inserted {
            answer.push(new_interval);
        }
        answer
    }
}
