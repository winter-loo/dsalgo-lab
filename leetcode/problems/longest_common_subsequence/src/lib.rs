pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        fn recur(t1: &[char], i1: usize, t2: &[char], i2: usize) -> i32 {
            let mut max_len = 0;
            for i in i1..t1.len() {
                let mut found = false;
                for j in i2..t2.len() {
                    if t1[i] == t2[j] {
                        found = true;
                        let m1 = 1 + recur(t1, i + 1, t2, j + 1);
                        let m2 = recur(t1, i + 1, t2, j);
                        let m = m1.max(m2);
                        max_len = max_len.max(m);
                        break;
                    }
                }
                if !found {
                    max_len = max_len.max(recur(t1, i + 1, t2, i2));
                }
            }
            max_len as i32
        }
        let (t1, t2): (Vec<char>, Vec<char>) = if text1.len() < text2.len() {
            (text1.chars().collect(), text2.chars().collect())
        } else {
            (text2.chars().collect(), text1.chars().collect())
        };
        recur(&t1, 0, &t2, 0)
    }
}
