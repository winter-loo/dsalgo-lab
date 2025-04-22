use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

struct Twitter {
    // TODO: Define fields
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        // TODO: Implement constructor
        Twitter {
            // Initialize fields
        }
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        // TODO: Implement post_tweet
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        // TODO: Implement get_news_feed
        vec![]
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        // TODO: Implement follow
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        // TODO: Implement unfollow
    }
}
