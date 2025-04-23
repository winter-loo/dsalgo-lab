# Approach

To find all unique combinations of candidates that sum up to the target, where each number can only be used once, we need to carefully handle duplicates in the candidates array.

## Strategy
Similar to the original Combination Sum problem, we'll use backtracking, but with additional considerations to handle duplicates and ensure each number is used at most once.

## Steps
1. Sort the candidates array. This is crucial for handling duplicates efficiently.
2. Use a recursive backtracking function to explore combinations:
   - If the current sum equals the target, add the current combination to the result.
   - If the current sum exceeds the target, stop exploring this path (backtrack).
   - Otherwise, try adding each candidate to the current combination and recursively explore.
3. To avoid duplicate combinations:
   - Skip duplicates at the same level of recursion.
   - Only consider each candidate once by incrementing the index in the recursive call.
4. To ensure each number is used at most once, start the next recursive call from the next index.

## Implementation Details
- Maintain a current combination and its running sum during the backtracking process.
- After sorting, check if the current candidate is the same as the previous one at the same level of recursion. If so, skip it to avoid duplicates.
- For each candidate, decide whether to include it in the current combination.
- Unlike the original problem, move to the next index in the recursive call to ensure each number is used at most once.

## Pseudocode
```
function backtrack(candidates, target, start_index, current_combination, current_sum, result):
    if current_sum == target:
        add current_combination to result
        return
    if current_sum > target:
        return
    
    for i from start_index to candidates.length - 1:
        // Skip duplicates at the same level
        if i > start_index and candidates[i] == candidates[i-1]:
            continue
        
        add candidates[i] to current_combination
        backtrack(candidates, target, i+1, current_combination, current_sum + candidates[i], result)
        remove candidates[i] from current_combination (backtrack)
```

## Time and Space Complexity
- **Time Complexity**: O(2^n) in the worst case, where n is the number of candidates. This is because each candidate can either be included or excluded.
- **Space Complexity**: O(target) for the recursion stack in the worst case.
