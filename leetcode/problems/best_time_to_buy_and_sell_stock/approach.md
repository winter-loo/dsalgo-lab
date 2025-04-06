# Approach: One-Pass with Tracking Minimum

## Intuition
To maximize profit, we want to buy at the lowest price and sell at the highest price after the buying day. We can solve this by keeping track of the minimum price seen so far and calculating the potential profit if we sell on the current day.

## Algorithm
1. Initialize two variables:
   - `min_price`: to keep track of the minimum price seen so far
   - `max_profit`: to keep track of the maximum profit that can be achieved

2. Iterate through the prices array once:
   - Update `min_price` if the current price is lower
   - Calculate the potential profit if we sell at the current price (current price - min_price)
   - Update `max_profit` if the potential profit is higher than the current `max_profit`

3. Return `max_profit` after the iteration

## Complexity Analysis
- Time Complexity: O(n) where n is the length of the prices array
  - We only need to iterate through the array once
  
- Space Complexity: O(1)
  - We only use a constant amount of extra space regardless of the input size

## Implementation Notes
- Initialize `min_price` to the first element or to a very large value
- Initialize `max_profit` to 0 (as we can always choose not to make any transaction)
- Be careful with edge cases like an empty array or an array with only one element
