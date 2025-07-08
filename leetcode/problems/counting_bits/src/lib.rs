pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        if n < 0 {
            return vec![];
        }
        if n == 0 {
            return vec![0];
        }
        if n == 1 {
            return vec![0, 1];
        }
        let mut answer = vec![0, 1];
        for i in 2..=n {
            let k = i & (i - 1);
            answer.push(answer[k as usize] + 1);
        }
        answer
    }
}
