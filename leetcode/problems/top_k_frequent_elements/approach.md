# Approach: Top K Frequent Elements

## Intuition
To find the K most frequent elements, we need to count the frequency of each element and then select the K elements with the highest frequencies.

## Approach
There are several ways to solve this problem:

### Hash Map + Heap Approach:
1. Use a hash map to count the frequency of each element in the array.
2. Use a min-heap of size k to keep track of the k elements with the highest frequencies.
3. Iterate through the hash map and add elements to the heap.
4. If the heap size exceeds k, remove the element with the lowest frequency.
5. After processing all elements, the heap will contain the k most frequent elements.

### Hash Map + Bucket Sort Approach:
1. Use a hash map to count the frequency of each element in the array.
2. Create an array of buckets where bucket[i] contains all elements that appear i times.
3. Iterate through the buckets from highest frequency to lowest.
4. Add elements to the result list until we have k elements.

### QuickSelect Approach:
1. Use a hash map to count the frequency of each element in the array.
2. Use the QuickSelect algorithm (similar to QuickSort) to find the k elements with the highest frequencies.

## Complexity Analysis

### Hash Map + Heap Approach:
- **Time Complexity**: O(n log k), where n is the number of elements in the array and k is the parameter.
- **Space Complexity**: O(n) for the hash map and O(k) for the heap.

### Hash Map + Bucket Sort Approach:
- **Time Complexity**: O(n), where n is the number of elements in the array.
- **Space Complexity**: O(n) for the hash map and buckets.

### QuickSelect Approach:
- **Time Complexity**: Average O(n), worst case O(n²).
- **Space Complexity**: O(n) for the hash map.

## Notes
The Bucket Sort approach is optimal for this problem as it achieves O(n) time complexity, satisfying the follow-up requirement for better than O(n log n) time complexity. 