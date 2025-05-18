pub struct Solution;

impl Solution {
    // Input: s = "leetcode", wordDict = ["leet","code"]
    //
    // 7    false
    // 6    false
    // 5    false
    // 4    true
    // 3    false
    // 2    false
    // 1    false
    // 0    true
    //
    // Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
    //
    // 8    g           false
    // 7    og          false
    // 6    dog         true
    // 5    ndog        false
    // 4    andog       false
    // 3    sandog      false
    // 2    tsandog     false
    // 1    atsandog    false
    // 0    catsandog   false|false
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len()];
        for i in (0..s.len()).rev() {
            for word in &word_dict {
                // println!("i={i} s[i..]={} word={word}", &s[i..]);
                if s[i..].starts_with(&word[..]) {
                    // println!("dp[i={i}]={}, next_i={}", dp[i], i + word.len());
                    dp[i] |= if i + word.len() < s.len() {
                        dp[i + word.len()]
                    } else {
                        true
                    };
                }
            }
        }
        dp[0]
    }
}
