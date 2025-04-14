# Approach: Time Based Key-Value Store

## Intuition
The key insight for this problem is to efficiently store and retrieve values based on both keys and timestamps. Since timestamps are strictly increasing for each key, we can use binary search to efficiently find the value with the largest timestamp that is less than or equal to the given timestamp.

## Multiple Approaches

### Approach 1: HashMap with Binary Search
1. Use a HashMap where each key maps to a list of (timestamp, value) pairs.
2. For the `set` operation, append the (timestamp, value) pair to the list for the given key.
3. For the `get` operation, use binary search to find the largest timestamp that is less than or equal to the given timestamp.

**Time Complexity**:
- `set`: O(1) - Appending to a list is a constant time operation.
- `get`: O(log n) - Binary search on a list of n timestamps.

**Space Complexity**: O(n) - Where n is the total number of key-value pairs stored.

### Approach 2: HashMap with TreeMap
1. Use a HashMap where each key maps to a TreeMap (or a similar data structure that maintains sorted order).
2. The TreeMap maps timestamps to values.
3. For the `set` operation, insert the (timestamp, value) pair into the TreeMap for the given key.
4. For the `get` operation, use the TreeMap's built-in method to find the largest key (timestamp) that is less than or equal to the given timestamp.

**Time Complexity**:
- `set`: O(log n) - Insertion into a TreeMap is a logarithmic time operation.
- `get`: O(log n) - Finding the floor key in a TreeMap is a logarithmic time operation.

**Space Complexity**: O(n) - Where n is the total number of key-value pairs stored.

### Approach 3: HashMap with Sorted Vector
1. Use a HashMap where each key maps to a vector of (timestamp, value) pairs.
2. Keep the vector sorted by timestamp.
3. For the `set` operation, since timestamps are strictly increasing, simply append the new pair to the end of the vector.
4. For the `get` operation, use binary search to find the largest timestamp that is less than or equal to the given timestamp.

**Time Complexity**:
- `set`: O(1) - Appending to a vector is a constant time operation.
- `get`: O(log n) - Binary search on a vector of n timestamps.

**Space Complexity**: O(n) - Where n is the total number of key-value pairs stored.

## Implementation Notes
- Since the problem states that timestamps for `set` operations are strictly increasing, we don't need to worry about maintaining sorted order when inserting new values. We can simply append them to the end of the list/vector.
- For the `get` operation, we need to find the largest timestamp that is less than or equal to the given timestamp. This is a classic binary search problem.
- If there are no values with a timestamp less than or equal to the given timestamp, we return an empty string.
- The problem guarantees that all timestamps are positive, so we don't need to handle negative timestamps.
- Be careful with edge cases, such as when the key doesn't exist or when there are no values with a timestamp less than or equal to the given timestamp.

## Topics
- Hash Table
- Binary Search
- Design
- String
