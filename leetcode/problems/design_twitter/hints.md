# Hints for Designing Twitter

1.  **Data Structures:** What data structures are suitable for storing user information, their followers, and their tweets?
    *   How can you efficiently store the relationship between followers and followees?
    *   How can you store tweets for each user, keeping in mind they need to be retrieved in reverse chronological order?
2.  **Timestamps:** How can you ensure tweets are ordered chronologically, especially when merging feeds from multiple users? You might need a global counter or timestamp.
3.  **News Feed Generation:** How can you efficiently retrieve the top 10 most recent tweets for a user's news feed?
    *   Consider the user's own tweets and the tweets of all users they follow.
    *   Think about merging sorted lists or using a heap/priority queue to find the most recent tweets across multiple sources.
4.  **Efficiency:** Consider the time complexity of each operation (`postTweet`, `getNewsFeed`, `follow`, `unfollow`). `getNewsFeed` is often the most complex. How can you optimize it?
