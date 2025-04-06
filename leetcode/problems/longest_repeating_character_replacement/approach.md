# Approach: Sliding Window with Character Frequency

## Intuition
The key insight for this problem is understanding that for a substring to be valid after at most `k` replacements, the number of characters that need to be replaced must be less than or equal to `k`. This means that in any valid substring of length `len`, if the most frequent character appears `max_freq` times, then we need to replace `len - max_freq` characters, and this value must be less than or equal to `k`.

## Algorithm
1. Use a sliding window approach with two pointers: `left` and `right`.
2. As we expand the window by moving the `right` pointer, we keep track of the frequency of each character in the current window.
3. For each window, calculate the number of replacements needed: `window_length - max_frequency`.
4. If the number of replacements needed is less than or equal to `k`, the window is valid, and we can potentially expand it.
5. If the number of replacements needed exceeds `k`, we need to shrink the window from the left until it becomes valid again.
6. Keep track of the maximum valid window length seen so far.

## Complexity Analysis
- Time Complexity: O(n) where n is the length of the string
  - We process each character at most twice (once when adding to the window and once when removing)
  
- Space Complexity: O(1)
  - We only need to store the frequency of uppercase English letters (at most 26 characters)

## Implementation Notes
- Since we're dealing with uppercase English letters only, we can use an array of size 26 instead of a hash map for character frequencies.
- The key formula is: `replacements_needed = window_length - max_frequency`.
- An optimization is that we don't always need to shrink the window when it becomes invalid. We can just keep track of the maximum valid window length and continue expanding.
- Be careful with edge cases like an empty string or when `k` is 0.
