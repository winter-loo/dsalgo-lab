# Approach: Group Anagrams

## Intuition
The key insight is to recognize that all anagrams have the same characters but in different orders. If we sort the characters of each string, anagrams will result in the same sorted string.

## Approach
1. Create a hash map where the key is the sorted version of a string and the value is a list of original strings that, when sorted, match that key.
2. Iterate through each string in the input array.
3. For each string, sort its characters and use it as a key in the hash map.
4. Append the original string to the list associated with its sorted key.
5. After processing all strings, return the values of the hash map, which are lists of grouped anagrams.

## Complexity Analysis
- **Time Complexity**: O(n * k log k), where n is the number of strings and k is the maximum length of a string. Sorting each string takes O(k log k) time, and we do this for n strings.
- **Space Complexity**: O(n * k) to store all the strings in the hash map.

## Alternative Approaches
For languages that don't have efficient string sorting, another approach is to use a character count as the key. Create a key where each position represents the count of a specific character. For example, for lowercase English letters, you could use a 26-length array where each position represents the count of a letter. 