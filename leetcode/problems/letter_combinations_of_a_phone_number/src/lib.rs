pub struct Solution;

impl Solution {
    const PHONE_KEY_MAP: [&str; 10] = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        fn solve(index: usize, digits: &[char], choices: &mut Vec<char>, results: &mut Vec<String>) {
            // println!("{choices:?}");
            if index >= digits.len() {
                results.push(choices.iter().collect::<String>());
                return;
            }
            let n = digits[index].to_digit(10).unwrap() as usize;
            for c in Solution::PHONE_KEY_MAP[n].chars() {
                choices.push(c);
                solve(index + 1, digits, choices, results);
                choices.pop();
            }
        }
        let mut choices = vec![];
        let mut results = vec![];
        solve(0, digits.chars().collect::<Vec<_>>().as_slice(), &mut choices, &mut results);
        results
    }

    // level-order traversal
    pub fn letter_combinations_level_order_traversal(digits: String) -> Vec<String> {
       use std::collections::VecDeque;
        if digits.is_empty() {
            return vec![];
        }

        let mut digits = digits.chars().map(|c| c.to_digit(10).unwrap() as usize);

        let mut myq = VecDeque::new();
        for c in Solution::PHONE_KEY_MAP[digits.next().unwrap()].chars() {
            myq.push_back(c.to_string());
        }

        // println!("myq={myq:?}");

        for n in digits {
            let qlen = myq.len();

            for _ in 0..qlen {
                let s = myq.pop_front().unwrap();
                for c in Solution::PHONE_KEY_MAP[n].chars() {
                    let mut t = s.clone();
                    t.push(c);
                    myq.push_back(t);
                }
            }
            // println!("myq={myq:?}");
        }

        myq.into_iter().collect()
    }
}
