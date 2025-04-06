# 271. Encode and Decode Strings

## Description

Design an algorithm to encode a list of strings to a string. The encoded string is then sent over the network and is decoded back to the original list of strings.

Implement the `Codec` class:

- `Codec()` Initializes the encoder/decoder.
- `String encode(List<String> strs)` Encodes a list of strings to a single string.
- `List<String> decode(String s)` Decodes a single string to a list of strings.

## Examples

**Example 1:**
```
Input: strs = ["Hello","World"]
Output: ["Hello","World"]
Explanation:
Machine 1:
Codec encoder = new Codec();
String encodedString = encoder.encode(strs);

Machine 2:
Codec decoder = new Codec();
List<String> decodedStrs = decoder.decode(encodedString);
// decodedStrs should be equal to the input strs
```

**Example 2:**
```
Input: strs = [""]
Output: [""]
```

## Constraints

- `1 <= strs.length <= 200`
- `0 <= strs[i].length <= 200`
- `strs[i]` contains any possible characters out of the 256 ASCII characters.

## Topics
- String
- Design
