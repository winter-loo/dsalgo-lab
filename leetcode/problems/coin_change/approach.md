# Approach

The "Coin Change" problem asks us to find the minimum number of coins needed to make up a given amount, where we have an unlimited supply of coins of different denominations.

## Strategy

This problem can be solved using:
1. **Dynamic Programming**: Build a solution from the bottom up
2. **Recursion with Memoization**: Top-down approach with caching
3. **Breadth-First Search (BFS)**: Treat it as a shortest path problem

## Dynamic Programming Approach
1. Create an array `dp` where `dp[i]` represents the minimum number of coins needed to make amount `i`
2. Initialize `dp[0] = 0` (it takes 0 coins to make amount 0)
3. Initialize all other values to a large number (e.g., amount + 1)
4. For each coin and each amount from 1 to the target amount:
   - If the current coin can be used (coin <= amount), update `dp[amount] = min(dp[amount], dp[amount - coin] + 1)`
5. Return `dp[amount]` if it's less than the initial large number, otherwise return -1

## BFS Approach
1. Start a BFS from amount 0
2. For each state, try using each coin denomination to reach a new amount
3. The first time we reach the target amount is the minimum number of coins needed

## Time and Space Complexity
- **Time Complexity**: O(amount * n) where n is the number of coin denominations
- **Space Complexity**: O(amount) for the DP array

## Pseudocode for Dynamic Programming Approach
```
function coinChange(coins, amount):
    dp = array of size amount+1, initialized to amount+1 (except dp[0] = 0)
    
    for coin in coins:
        for i from coin to amount:
            dp[i] = min(dp[i], dp[i - coin] + 1)
    
    if dp[amount] > amount:
        return -1
    else:
        return dp[amount]
```

## Pseudocode for BFS Approach
```
function coinChange(coins, amount):
    if amount == 0:
        return 0
    
    queue = new Queue()
    queue.push(0)
    visited = set containing 0
    level = 0
    
    while queue is not empty:
        size = queue.size()
        level++
        
        for i from 1 to size:
            current = queue.pop()
            
            for coin in coins:
                next = current + coin
                
                if next == amount:
                    return level
                
                if next < amount and next not in visited:
                    visited.add(next)
                    queue.push(next)
    
    return -1
```
