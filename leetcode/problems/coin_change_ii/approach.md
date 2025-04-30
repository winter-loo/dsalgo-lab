# Approach

The "Coin Change II" problem asks us to find the number of different ways to make up a given amount using a set of coins, where each coin can be used multiple times.

## Strategy

This problem can be solved using:
1. **Dynamic Programming**: Build a solution using a 1D or 2D DP table
2. **Recursion with Memoization**: Top-down approach with caching

## Dynamic Programming Approach (2D)
1. Create a 2D DP table where `dp[i][j]` represents the number of ways to make amount `j` using the first `i` types of coins
2. Initialize `dp[0][0] = 1` (there's 1 way to make amount 0 with 0 coins - by not using any coin)
3. Initialize the rest of the first row to 0 (there's 0 ways to make a non-zero amount with 0 coins)
4. For each coin and each amount:
   - If we don't use the current coin: `dp[i][j] = dp[i-1][j]`
   - If we use the current coin: `dp[i][j] += dp[i][j-coins[i-1]]` (if j >= coins[i-1])
5. Return `dp[n][amount]`

## Dynamic Programming Approach (1D)
1. Create a 1D DP array where `dp[j]` represents the number of ways to make amount `j`
2. Initialize `dp[0] = 1` (there's 1 way to make amount 0 - by not using any coin)
3. For each coin and each amount from the coin value to the target amount:
   - Update `dp[j] += dp[j-coin]`
4. Return `dp[amount]`

## Time and Space Complexity
- **Time Complexity**: O(n * amount) where n is the number of coin denominations
- **Space Complexity**: 
  - 2D DP: O(n * amount)
  - 1D DP: O(amount)

## Pseudocode for Dynamic Programming Approach (2D)
```
function change(amount, coins):
    n = length(coins)
    dp = 2D array of size (n+1) x (amount+1), initialized to 0
    dp[0][0] = 1
    
    for i from 1 to n:
        dp[i][0] = 1  // There's 1 way to make amount 0 with any number of coins
        for j from 1 to amount:
            // Don't use the current coin
            dp[i][j] = dp[i-1][j]
            
            // Use the current coin if possible
            if j >= coins[i-1]:
                dp[i][j] += dp[i][j-coins[i-1]]
    
    return dp[n][amount]
```

## Pseudocode for Dynamic Programming Approach (1D)
```
function change(amount, coins):
    dp = array of size amount+1, initialized to 0
    dp[0] = 1
    
    for coin in coins:
        for j from coin to amount:
            dp[j] += dp[j-coin]
    
    return dp[amount]
```
