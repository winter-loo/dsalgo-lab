# Approach

The "Hand of Straights" problem asks us to determine if we can
rearrange a hand of cards into groups of consecutive cards, where
each group has a fixed size.

## Strategy

This problem can be solved using:
1. **Greedy Algorithm**: Process cards in sorted order and form groups
2. **Hash Map**: Keep track of the frequency of each card

## Greedy Approach
1. Count the frequency of each card value using a hash map or a sorted map
2. Process the cards in ascending order (to ensure we're forming consecutive groups)
3. For each card value:
   - If its frequency is 0, skip it
   - If its frequency is > 0, try to form a group starting with this card
   - Check if we can find the next `groupSize - 1` consecutive cards
   - If any card in the sequence is missing, return false
   - Otherwise, decrease the frequency of each card in the group
4. If we can process all cards, return true

## Time and Space Complexity
- **Time Complexity**: O(n log n) where n is the number of cards
  - Sorting the cards takes O(n log n)
  - Processing each card takes O(n)
- **Space Complexity**: O(n) for storing the frequency of each card

## Pseudocode for Greedy Approach
```
function isNStraightHand(hand, groupSize):
    n = length of hand
    
    // Check if the hand can be divided into groups
    if n % groupSize != 0:
        return false
    
    // Count the frequency of each card
    freq = empty map
    for card in hand:
        freq[card] = freq[card] + 1 or 1
    
    // Process cards in ascending order
    sort hand
    for card in sorted(hand):
        if freq[card] == 0:
            continue
        
        // Try to form a group starting with this card
        for i from 0 to groupSize - 1:
            currentCard = card + i
            if freq[currentCard] == 0:
                return false
            freq[currentCard] = freq[currentCard] - 1
    
    return true
```

## Alternative Approach: Using a Sorted Map
1. Use a sorted map (like TreeMap in Java or SortedDict in Python) to store card frequencies
2. Process the smallest card first, then form groups
3. This approach avoids explicit sorting

This approach is more efficient for certain inputs, especially when there are many duplicate cards.
