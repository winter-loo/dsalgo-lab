# Distinct Subsequences

[LeetCode Link](https://leetcode.com/problems/distinct-subsequences/)

Given two strings `s` and `t`, return the number of distinct subsequences of `s` which equals `t`.

A string's **subsequence** is a new string formed from the original string by deleting some (can be none) of the characters without disturbing the remaining characters' relative positions. (i.e., `"ACE"` is a subsequence of `"ABCDE"` while `"AEC"` is not).

The answer is guaranteed to fit in a 32-bit integer.

**Example 1:**
```
Input: s = "rabbbit", t = "rabbit"
Output: 3
Explanation:
As shown below, there are 3 ways you can get "rabbit" from "rabbbit".
rabbbit
^^^^ ^^
rabbbit
^^ ^^^^
rabbbit
^^^ ^^^
```

**Example 2:**
```
Input: s = "babgbag", t = "bag"
Output: 5
Explanation:
As shown below, there are 5 ways you can get "bag" from "babgbag".
babgbag
^^ ^
babgbag
^^    ^
babgbag
^    ^^
babgbag
  ^  ^^
babgbag
    ^^^
```

**Constraints:**
- `1 <= s.length, t.length <= 1000`
- `s` and `t` consist of lowercase English letters.
