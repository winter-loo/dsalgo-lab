use generate_parentheses::Solution;
use std::collections::HashSet;

fn main() {
    let v = Solution::generate_parenthesis(3);
    let s: HashSet<_> = v.iter().collect();
    println!("{:#?} len={}", v, v.len());
    println!("{:#?} len={}", s, s.len());
}
