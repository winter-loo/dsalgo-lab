# Hints for Climbing Stairs

1. Try to think about the problem recursively. How many ways can you reach the nth step?

2. To reach the nth step, you can either come from the (n-1)th step by taking 1 step, or from the (n-2)th step by taking 2 steps.

3. This means the number of ways to reach the nth step is the sum of the number of ways to reach the (n-1)th step and the (n-2)th step.

4. This is a classic dynamic programming problem. Consider using an array where dp\[i\] represents the number of ways to reach the ith step.

5. The base cases are simple: there is 1 way to climb 1 step and 2 ways to climb 2 steps.

6. For optimization, notice that you only need to keep track of the previous two values at each step, not the entire array.

7. The pattern of this problem follows the Fibonacci sequence, where each number is the sum of the two preceding ones.

8. Be careful with the base cases and indexing when implementing your solution.
