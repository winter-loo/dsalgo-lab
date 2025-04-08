pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        if s.len() < 2 || k >= s.len() as i32 {
            return s.len() as i32;
        }

        let s = s.into_bytes();
        // Use a fixed-size array for frequency counting (ASCII uppercase letters: A-Z)
        let mut freq = [0; 26];
        
        let mut max_len = 0;
        let mut max_freq = 0;
        let mut left = 0;
        
        // Single pass through the string
        for right in 0..s.len() {
            // Calculate index for current character (assuming uppercase A-Z)
            let char_idx = (s[right] - b'A') as usize;
            
            // Update frequency and max frequency
            freq[char_idx] += 1;
            max_freq = max_freq.max(freq[char_idx]);
            
            // If current window requires more than k replacements
            let window_size = right - left + 1;
            if window_size as i32 - max_freq > k {
                // Shrink window from left
                freq[(s[left] - b'A') as usize] -= 1;
                left += 1;
            }
            
            // Current window is valid, update max length
            max_len = max_len.max(right - left + 1);
        }
        
        max_len as i32
    }

    pub fn character_replacement_attempt_1_brute_force(s: String, k: i32) -> i32 {
        if s.len() < 2 || k >= s.len() as i32 {
            return s.len() as i32;
        }
        // for (i, c) in s.chars().enumerate() {
        //     println!("{i}:{c}");
        // }

        let s = s.into_bytes();
        let mut ans = 0;

        let (mut l, mut r, mut changes) = (0, 0, 0);
        while l < s.len() {
            if s[r] != s[l] {
                changes += 1;
            }
            // println!("before: l={l} r={r} changes={changes}");
            if changes > k {
                // invalid window
                while s[l] == s[l + 1] {
                    l += 1;
                }
                // move the left boundary
                l += 1;
                // reset the window right boundary to its left
                // as we dont know how many different characters with the new left boundary character
                r = l;
                changes = 0;
            } else {
                ans = ans.max(r - l + 1);
                r += 1; // expand the window right boundary
            }

            // expand beyond the right boundary
            if r >= s.len() {
                if changes < k {
                    // right boundary reached the limit and then go left
                    // println!("ans={ans} k={k} changes={changes} l={l} r={r}");
                    ans = ans.max((r - l) + l.min((k - changes) as usize));
                }
                l += 1;
                r = l;
                changes = 0;
            }
        }

        ans as i32
    }
}
