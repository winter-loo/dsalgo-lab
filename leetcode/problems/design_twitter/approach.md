# Approach for Designing Twitter

## Core Data Structures

1.  **User Follows:** A `HashMap<UserId, HashSet<UserId>>` can store who each user follows. `follows[follower_id]` would be a set containing all `followee_id`s.
2.  **User Tweets:** A `HashMap<UserId, Vec<(Timestamp, TweetId)>>` can store tweets for each user. Store them with a timestamp to maintain order. A global timestamp counter can be incremented with each post.
3.  **Global Timestamp:** An integer counter (e.g., `time`) incremented every time `postTweet` is called. This provides a simple way to order tweets chronologically.

## Operations

1.  **`new()`:** Initialize the HashMaps and the global timestamp.
2.  **`postTweet(userId, tweetId)`:**
    *   Increment the global timestamp.
    *   Get the tweet list for `userId`. If the user doesn't exist in the map, create an entry.
    *   Add `(current_timestamp, tweetId)` to the user's tweet list.
3.  **`follow(followerId, followeeId)`:**
    *   Get the follow set for `followerId`. If it doesn't exist, create it.
    *   Add `followeeId` to the set. Ensure a user cannot follow themselves (optional, based on typical requirements).
4.  **`unfollow(followerId, followeeId)`:**
    *   Get the follow set for `followerId`.
    *   If the set exists, remove `followeeId` from it.
5.  **`getNewsFeed(userId)`:**
    *   Identify all relevant users: `userId` themselves and all `followeeId`s in `follows[userId]`.
    *   Gather the most recent tweets from *all* these users. Since we only need the top 10 overall, we don't need all tweets from everyone.
    *   **Using a Max-Heap:** A good approach is to use a max-heap (priority queue) storing `(Timestamp, TweetId)`. 
        *   Initialize the heap by adding the latest tweet (if any) from the user and each person they follow.
        *   Repeatedly extract the tweet with the largest timestamp from the heap (up to 10 times).
        *   Each time a tweet is extracted from a user's list, add the *next* most recent tweet from that same user's list back into the heap.
        *   This efficiently merges the sorted tweet lists to find the overall top 10.
    *   Collect the `tweetId`s from the extracted heap elements.

## Complexity

*   **Time:**
    *   `postTweet`, `follow`, `unfollow`: O(1) average time (assuming HashMap operations are O(1) on average).
    *   `getNewsFeed`: If a user follows `F` users, and we need `K` tweets (here `K=10`), using a min-heap of size `K` or merging `F+1` lists takes roughly O(K * log(F+1)) or O(K * F) depending on the exact merging strategy. If using the max-heap approach described above, adding initial tweets is O(F log F), and then extracting K tweets involves K heap operations, each potentially adding another element, leading roughly to O(K log F).
*   **Space:**
    *   O(Total Users + Total Follows + Total Tweets) to store the data structures.
