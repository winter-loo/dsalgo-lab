# Approach: Product of Array Except Self

## Intuition
The key insight is to calculate the product of all elements to the left and to the right of each position separately, then multiply these values to get the final result.

## Approach
1. Create a result array of the same length as the input array.
2. Perform two passes through the array:
   - First Pass (Left to Right): Calculate the product of all elements to the left of each position.
   - Second Pass (Right to Left): Calculate the product of all elements to the right of each position and multiply it with the left product calculated in the first pass.
3. The resulting array will contain the product of all elements except the element at the corresponding position.

This approach avoids using division and achieves O(n) time complexity with O(1) extra space (not counting the output array).

## Complexity Analysis
- **Time Complexity**: O(n), where n is the number of elements in the array. We make two passes through the array.
- **Space Complexity**: O(1) extra space, not counting the output array.

## Edge Cases
- If the array contains zeros, elements at other positions may result in zero products.
- Negative numbers don't require special handling since multiplication works the same regardless of sign. 