# Approach: Sliding Window with Hash Set

## Intuition
The key insight for this problem is to use a sliding window approach to track the current substring without repeating characters. When we encounter a character that's already in our current substring, we need to adjust the window by moving the left pointer.

## Algorithm
1. Use a sliding window approach with two pointers: `left` and `right`.
2. As we expand the window by moving the `right` pointer, we check if the character at the `right` pointer is already in our current substring.
3. If it's not, we add it to our set of characters and continue expanding the window.
4. If it is, we need to adjust the `left` pointer to exclude the first occurrence of this character and all characters before it.
5. At each step, we calculate the length of the current window and keep track of the maximum length seen so far.

## Complexity Analysis
- Time Complexity: O(n) where n is the length of the string
  - We process each character at most twice (once when adding to the window and once when removing)
  
- Space Complexity: O(min(m, n)) where m is the size of the character set
  - In the worst case, we might need to store all unique characters in the string

## Implementation Notes
- We can use a hash set to keep track of characters in the current window.
- For optimization, we can use a hash map to store the index of each character's last occurrence, which allows us to jump the left pointer directly to the right position.
- Be careful with edge cases like an empty string.
- Remember that the problem asks for the length of the longest substring, not the substring itself.
