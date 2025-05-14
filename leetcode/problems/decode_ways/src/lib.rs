pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        Solution::num_decodings_with_memoization(s)
    }

    pub fn num_decodings_with_memoization(s: String) -> i32 {
        fn traverse(s: &[char], index: usize, sum: u32, mem: &mut [u32]) -> u32 {
            println!("index={index} sum={sum} mem={mem:?}");
            if index >= s.len() {
                let n = if sum > 0 && sum <= 26 { 1 } else { 0 };
                println!("index={index} sum={sum} mem={mem:?}, returning n={n}");
                return n;
            }

            if sum == 0 && mem[index] != 0 {
                println!("index={index} sum={sum} mem={mem:?}, returning n={}", mem[index]);
                return mem[index];
            }

            let n = traverse(s, index + 1, 0, mem);

            let current_digit = s[index].to_digit(10).unwrap_or(0);
            let new_sum = sum * 10 + current_digit;
            if new_sum <= 0 || new_sum > 26 {
                println!("--- invalid, index={index} sum={sum} mem={mem:?}, returning 0");
                return 0;
            }

            mem[index] = n + traverse(s, index + 1, new_sum, mem);
            println!("FINAL => index={index} sum={sum} mem={mem:?}, returning n={}", mem[index]);
            mem[index]
        }
        let s: Vec<_> = s.chars().collect();
        let mut mem = vec![0; s.len()];
        traverse(&s, 0, 0, &mut mem) as i32
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
