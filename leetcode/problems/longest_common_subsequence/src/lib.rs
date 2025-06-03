pub struct Solution;

impl Solution {
    // ace aecde
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (t1, t2): (Vec<char>, Vec<char>) = if text1.len() < text2.len() {
            (text1.chars().collect(), text2.chars().collect())
        } else {
            (text2.chars().collect(), text1.chars().collect())
        };

        let mut j = 0;
        let mut max_len = 0;
        let mut len = 0;
        let mut found = false;
        for i in 0..t1.len() {
            for k in j..t2.len() {
                if t1[i] == t2[k] {
                    found = true;
                    j += 1;
                    len += 1;
                    break;
                }
            }
            max_len = max_len.max(len);
            if !found {
                j = 0;
                len = 0;
            }
        }
        max_len as i32
    }
}
