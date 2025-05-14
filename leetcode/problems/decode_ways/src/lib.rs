pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s: Vec<_> = s.chars().collect();
        let mut count = 0;
        traverse(&s, 0, 0, &mut count);
        count as i32
    }
}

// Time Limit Exceeded version
// https://excalidraw.com/#json=jFRTgrklo5mLYgWSL9b_4,YCYDUIuUWbJNNAtxf0WYMQ
fn traverse_with_memoization(s: &[char], index: usize, sum: u32, count: &mut usize) {
    println!("index={index} sum={sum}");
    if index >= s.len() {
        if sum > 0 && sum <= 26 {
            *count += 1;
        }
        return;
    }
    let sum = s[index].to_digit(10).map(|n| sum * 10 + n).unwrap_or(0);
    if sum <= 0 || sum > 26 {
        return;
    }
    traverse(s, index + 1, sum, count);
    traverse(s, index + 1, 0, count);
}

// Time Limit Exceeded version
// https://excalidraw.com/#json=18SkzARuhP4oxVhFqGyN_,0lMgdgeWP4wXR2thy9ofBg
fn traverse(s: &[char], index: usize, sum: u32, count: &mut usize) {
    println!("index={index} sum={sum}");
    if index >= s.len() {
        if sum > 0 && sum <= 26 {
            *count += 1;
        }
        return;
    }
    let sum = s[index].to_digit(10).map(|n| sum * 10 + n).unwrap_or(0);
    if sum <= 0 || sum > 26 {
        return;
    }
    traverse(s, index + 1, sum, count);
    traverse(s, index + 1, 0, count);
}
