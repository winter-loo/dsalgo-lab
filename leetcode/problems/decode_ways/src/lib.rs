pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        Solution::num_decodings_dp(s)
    }

    // f(s, i) =
    // f(s, i + 1) if s[i] != 0
    // +
    // f(s, i + 2) if s[i] * 10 + s[i + 1] in [1,26]
    //
    // 123
    //
    // g(3) = 1 as 23 in [1,26]
    // g(2) = 1 as 3 != 0
    // g(1) = g(2) + g(3) = 1 + 1 = 2
    // g(0) = g(1) + g(2) = 2 + 1 = 3
    //
    // 101
    //
    // g(3) = 0
    // g(2) = 1
    // g(1) = 0
    // g(0) = g(1) + g(2) = 1
    //
    // 10101
    //
    // g(5) = 0 as '01' is invalid
    // g(4) = 1 as '1' is valid
    // g(3) = 0 as '0' is invalid
    // g(2) = g(3) + g(4) = 1
    // g(1) = 0 as '0' is invalid
    // g(0) = g(1) + g(2) = 1
    pub fn num_decodings_dp(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let s: Vec<_> = s.chars().collect();
        if s.len() == 1 {
            return if s[0] != '0' { 1 } else { 0 };
        }

        // set initial values
        let last_digit = s[s.len() - 1].to_digit(10).unwrap_or(0);
        let next_to_last_digit = s[s.len() - 2].to_digit(10).unwrap_or(0);
        let sum = if next_to_last_digit == 0 {
            0
        } else {
            next_to_last_digit * 10 + last_digit
        };
        // order: f4, f3, f2, f1
        let mut f1 = if sum > 0 && sum <= 26 { 1 } else { 0 };
        let mut f2 = if last_digit == 0 { 0 } else { 1 };

        // dynamically calculating the answer
        let mut ans = 0;
        for i in (0..s.len() - 1).rev() {
            // println!("f1={f1} f2={f2} ans={ans}");
            let digit = s[i].to_digit(10).unwrap_or(0);
            // println!("current digit={digit}");
            if digit == 0 {
                ans = 0;
            } else {
                ans = f2;

                let prev_digit = s[i + 1].to_digit(10).unwrap_or(0);
                let sum = digit * 10 + prev_digit;
                if sum > 0 && sum <= 26 {
                    ans += f1;
                }
            }

            f1 = f2;
            f2 = ans;
            // println!("done f1={f1} f2={f2} ans={ans}");
        }
        ans as i32
    }

    pub fn num_decodings_2(s: String) -> i32 {
        // in-order traversal
        fn traverse(s: &[char], index: usize, sum: u32) -> u32 {
            if index >= s.len() {
                return if sum > 0 && sum <= 26 { 1 } else { 0 };
            }

            let left = traverse(s, index + 1, 0);

            let current_digit = s[index].to_digit(10).unwrap_or(0);
            let sum = sum * 10 + current_digit;
            // stop exploring when reaching an invalid state
            if sum <= 0 || sum > 26 {
                return 0;
            }

            let right = traverse(s, index + 1, sum);

            left + right
        }
        let s: Vec<_> = s.chars().collect();
        traverse(&s, 0, 0) as i32
    }

    pub fn num_decodings_1(s: String) -> i32 {
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
        let s: Vec<_> = s.chars().collect();
        let mut count = 0;
        traverse(&s, 0, 0, &mut count);
        count as i32
    }
}
