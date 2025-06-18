# Hints for Single Number

1. Think about how to identify a single element when all other elements appear exactly twice.

2. Consider using bitwise operations. XOR (exclusive OR) has properties that can be useful for this problem.

3. Remember that XOR of a number with itself is 0: `a ⊕ a = 0`.

4. Also, XOR of a number with 0 is the number itself: `a ⊕ 0 = a`.

5. XOR is commutative and associative, meaning the order of operations doesn't matter: `a ⊕ b ⊕ a = b`.

6. If you XOR all numbers in the array, the duplicate elements will cancel each other out (become 0), and you'll be left with the single element.

7. The solution should have O(n) time complexity and O(1) space complexity.

8. You don't need any extra data structures like hash maps or sets to solve this problem optimally.

9. The solution is a single pass through the array with a single variable to keep track of the result.

10. This approach works because of the specific constraint that every element appears exactly twice except for one element.
