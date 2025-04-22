# Approach

To find the least number of units of time required to complete all tasks with a cooldown period, we need to consider the frequency of each task and the constraints imposed by the cooldown period.

## Strategy
- The task with the highest frequency will determine the minimum time required.
- We need to arrange tasks to maximize CPU utilization and minimize idle time.
- The key insight is to prioritize tasks with higher frequencies.

## Steps
1. Count the frequency of each task.
2. Identify the task with the maximum frequency (max_freq).
3. Calculate the minimum time required based on the task with the highest frequency:
   - The last occurrence of the most frequent task doesn't need idle slots after it.
   - For each occurrence of the most frequent task (except the last one), we need to allocate (n+1) slots (1 for the task itself and n for the cooldown).
   - This gives us: (max_freq - 1) * (n + 1) + (count of tasks with max_freq) as a baseline.
4. The actual answer is the maximum of this calculated value and the total number of tasks (since we can't have fewer slots than tasks).

## Implementation Options
- **Greedy Approach**: Always choose the task with the highest remaining frequency that is not in cooldown.
- **Mathematical Approach**: Use the formula described above to calculate the minimum time directly.

## Edge Cases
- If n = 0, the answer is simply the total number of tasks.
- If all tasks have the same frequency, we need to account for all of them in the final calculation.

// TODO: Add time and space complexity analysis.
