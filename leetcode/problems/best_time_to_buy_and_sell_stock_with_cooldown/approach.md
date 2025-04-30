# Approach

The "Best Time to Buy and Sell Stock with Cooldown" problem asks us to find the maximum profit we can make by buying and selling stocks, with the constraint that after selling, we must wait one day before buying again.

## Strategy

This problem can be solved using:
1. **Dynamic Programming**: Build a solution using state transitions
2. **State Machine**: Model the problem as a state machine with different states

## Dynamic Programming Approach
1. Define three states:
   - `hold[i]`: Maximum profit on day i if we are holding a stock
   - `sold[i]`: Maximum profit on day i if we just sold a stock
   - `rest[i]`: Maximum profit on day i if we are in the cooldown period or doing nothing
2. State transitions:
   - `hold[i] = max(hold[i-1], rest[i-1] - prices[i])` (either continue holding or buy a new stock)
   - `sold[i] = hold[i-1] + prices[i]` (sell the stock we were holding)
   - `rest[i] = max(rest[i-1], sold[i-1])` (either continue resting or enter cooldown after selling)
3. Initialize:
   - `hold[0] = -prices[0]` (buy the stock on day 0)
   - `sold[0] = 0` (cannot sell on day 0)
   - `rest[0] = 0` (no profit if doing nothing on day 0)
4. Return `max(sold[n-1], rest[n-1])` (we cannot end with holding a stock for maximum profit)

## Space-Optimized Approach
1. Since we only need the previous day's states to calculate the current day's states, we can optimize space to O(1)
2. Use variables to track the previous day's states and update them for each day

## Time and Space Complexity
- **Time Complexity**: O(n) where n is the length of the prices array
- **Space Complexity**: 
  - Full DP: O(n)
  - Optimized: O(1)

## Pseudocode for Dynamic Programming Approach
```
function maxProfit(prices):
    n = length(prices)
    if n <= 1:
        return 0
    
    hold = array of size n
    sold = array of size n
    rest = array of size n
    
    hold[0] = -prices[0]
    sold[0] = 0
    rest[0] = 0
    
    for i from 1 to n-1:
        hold[i] = max(hold[i-1], rest[i-1] - prices[i])
        sold[i] = hold[i-1] + prices[i]
        rest[i] = max(rest[i-1], sold[i-1])
    
    return max(sold[n-1], rest[n-1])
```

## Pseudocode for Space-Optimized Approach
```
function maxProfit(prices):
    n = length(prices)
    if n <= 1:
        return 0
    
    hold = -prices[0]
    sold = 0
    rest = 0
    
    for i from 1 to n-1:
        prevHold = hold
        prevSold = sold
        prevRest = rest
        
        hold = max(prevHold, prevRest - prices[i])
        sold = prevHold + prices[i]
        rest = max(prevRest, prevSold)
    
    return max(sold, rest)
```
