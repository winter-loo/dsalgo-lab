pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        // Early return for edge cases
        if s.len() < t.len() || s.is_empty() || t.is_empty() {
            return String::from("");
        }

        // Count characters in pattern t
        let mut t_counts: HashMap<char, i32> = HashMap::new();
        for c in t.chars() {
            *t_counts.entry(c).or_insert(0) += 1;
        }
        
        let s_chars: Vec<char> = s.chars().collect();
        let mut window: HashMap<char, i32> = HashMap::new();
        
        // Track how many unique characters we need to match
        let required_chars = t_counts.len();
        let mut formed_chars = 0;
        
        // Track the minimum window
        let mut min_len = usize::MAX;
        let mut min_start = 0;
        
        // Sliding window pointers
        let mut left = 0;
        let mut right = 0;
        
        // Expand window to the right
        while right < s_chars.len() {
            // Add current character to window
            let c = s_chars[right];
            *window.entry(c).or_insert(0) += 1;
            
            // Check if this character helps us match a required character
            if t_counts.contains_key(&c) && window.get(&c) == t_counts.get(&c) {
                formed_chars += 1;
            }
            
            // Try to contract window from the left while maintaining all required characters
            while left <= right && formed_chars == required_chars {
                let current_len = right - left + 1;
                
                // Update minimum window if current is smaller
                if current_len < min_len {
                    min_len = current_len;
                    min_start = left;
                }
                
                // Remove leftmost character from window
                let left_char = s_chars[left];
                *window.get_mut(&left_char).unwrap() -= 1;
                
                // Check if removing this character breaks a match
                if t_counts.contains_key(&left_char) && window.get(&left_char) < t_counts.get(&left_char) {
                    formed_chars -= 1;
                }
                
                // Move left pointer
                left += 1;
            }
            
            // Move right pointer
            right += 1;
        }
        
        // Return the minimum window substring or empty string if not found
        if min_len == usize::MAX {
            String::from("")
        } else {
            s_chars[min_start..min_start + min_len].iter().collect()
        }
    }
}
