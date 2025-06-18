# Approach

The "Partition Labels" problem asks us to partition a string into as many parts as possible such that each letter appears in at most one part.

## Understanding the Problem

For each letter in the string, all of its occurrences must be in the same partition. This means that if a letter appears at positions i and j, then all characters from positions i to j must be in the same partition.

## Greedy Approach

1. First, find the last occurrence of each character in the string.
2. Then, iterate through the string and keep track of the current partition's end.
3. For each character, update the partition end to be the maximum of the current end and the last occurrence of the current character.
4. When we reach the current partition's end, we've completed a partition.

## Time and Space Complexity

- **Time Complexity**: O(n), where n is the length of the string. We iterate through the string twice.
- **Space Complexity**: O(1), as we only need to store the last occurrence of each character (at most 26 for lowercase English letters).

## Pseudocode

```
function partitionLabels(s):
    // Find the last occurrence of each character
    lastOccurrence = empty map
    for i from 0 to length(s) - 1:
        lastOccurrence[s[i]] = i
    
    // Partition the string
    result = empty list
    start = 0
    end = 0
    
    for i from 0 to length(s) - 1:
        end = max(end, lastOccurrence[s[i]])
        
        if i == end:
            // We've reached the end of the current partition
            result.append(end - start + 1)
            start = i + 1
    
    return result
```

## Step-by-Step Example

Let's trace through the algorithm with the example string "ababcbacadefegdehijhklij":

1. Find the last occurrence of each character:
   - 'a': 8
   - 'b': 5
   - 'c': 7
   - 'd': 14
   - 'e': 15
   - 'f': 11
   - 'g': 13
   - 'h': 19
   - 'i': 22
   - 'j': 23
   - 'k': 20
   - 'l': 21

2. Iterate through the string:
   - For 'a' at index 0: end = max(0, 8) = 8
   - For 'b' at index 1: end = max(8, 5) = 8
   - ...
   - For 'a' at index 8: end = max(8, 8) = 8
     - We've reached the end of the first partition: length = 8 - 0 + 1 = 9
   - ...
   - For 'e' at index 15: end = max(15, 15) = 15
     - We've reached the end of the second partition: length = 15 - 9 + 1 = 7
   - ...
   - For 'j' at index 23: end = max(23, 23) = 23
     - We've reached the end of the third partition: length = 23 - 16 + 1 = 8

3. Result: [9, 7, 8]

This approach ensures that each character appears in exactly one partition, and we create as many partitions as possible.
