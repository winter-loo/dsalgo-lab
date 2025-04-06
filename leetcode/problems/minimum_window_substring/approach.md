# Approach: Sliding Window with Two Pointers

## Intuition
This problem is a classic example of the sliding window technique. We need to find the smallest substring in `s` that contains all characters from `t`, including duplicates.

## Algorithm
1. Create two hash maps (or frequency counters):
   - `target_freq`: to store the frequency of each character in string `t`
   - `window_freq`: to track the frequency of characters in our current window in string `s`

2. Use two pointers, `left` and `right`, to maintain our sliding window:
   - `right` expands the window by including characters
   - `left` contracts the window when possible to minimize its size

3. Track a variable `formed` that counts how many unique characters from `t` have their required frequency in our current window.

4. Expand the window by moving the `right` pointer until we have all required characters.

5. Once we have all required characters, try to minimize the window by moving the `left` pointer as far right as possible without losing our required characters.

6. Keep track of the minimum window found so far.

## Complexity Analysis
- Time Complexity: O(m + n) where m is the length of string `s` and n is the length of string `t`
  - We process each character of both strings at most once
  
- Space Complexity: O(k) where k is the size of the character set
  - In the worst case, both `target_freq` and `window_freq` will store all possible characters
  - Since we're dealing with English letters, this is O(1) in practice

## Implementation Notes
- Use a HashMap to track character frequencies
- Be careful with the edge cases:
  - When `t` is empty
  - When `s` is shorter than `t`
  - When no valid window exists
- Optimize by only considering characters that appear in `t`
