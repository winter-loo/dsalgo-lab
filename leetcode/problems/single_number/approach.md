# Approach

The "Single Number" problem asks us to find the single element in an array where every other element appears exactly twice.

## Understanding the Problem

We have an array of integers where every element appears twice except for one element, which appears only once. We need to find that single element. Additionally, we need to solve this with linear time complexity and constant space complexity.

## XOR Approach

The key insight for this problem is to use the XOR (exclusive OR) operation. XOR has the following properties:
- `a ⊕ 0 = a` (XOR with 0 returns the original number)
- `a ⊕ a = 0` (XOR of a number with itself returns 0)
- `a ⊕ b ⊕ a = b` (XOR is commutative and associative)

Using these properties, if we XOR all numbers in the array, the duplicate elements will cancel each other out (become 0), and we'll be left with the single element.

## Time and Space Complexity

- **Time Complexity**: O(n), where n is the length of the array. We iterate through the array once.
- **Space Complexity**: O(1), as we only use a single variable to store the result.

## Pseudocode

```
function singleNumber(nums):
    result = 0
    
    for num in nums:
        result = result XOR num
    
    return result
```

## Step-by-Step Example

Let's trace through the algorithm with the example: nums = [4,1,2,1,2]

1. Initialize result = 0.
2. Iterate through the array:
   - result = 0 ⊕ 4 = 4
   - result = 4 ⊕ 1 = 5
   - result = 5 ⊕ 2 = 7
   - result = 7 ⊕ 1 = 6 (since 1 ⊕ 1 = 0, and 7 ⊕ 0 = 7)
   - result = 6 ⊕ 2 = 4 (since 2 ⊕ 2 = 0, and 6 ⊕ 0 = 6)
3. Return result = 4.

## Alternative Approaches

### Hash Set Approach

Another approach is to use a hash set:
1. Iterate through the array.
2. If the current number is in the set, remove it.
3. If the current number is not in the set, add it.
4. At the end, the set will contain only the single element.

However, this approach uses O(n) space, which doesn't meet the constant space requirement.

### Mathematical Approach

We can also use a mathematical approach:
1. Find the sum of all unique elements in the array (let's call it uniqueSum).
2. Find the sum of all elements in the array (let's call it totalSum).
3. The single element is (2 * uniqueSum - totalSum).

However, this approach also requires O(n) space to store the unique elements, which doesn't meet the constant space requirement.

The XOR approach is the most elegant and efficient solution for this problem, as it meets both the linear time and constant space requirements.
