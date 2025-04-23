use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

#[derive(Debug)]
pub struct Twitter {
    // a list of tweet_id
    tweets_store: Vec<i32>,
    // map user_id to the index of tweet_id in the tweets_store
    tweets: HashMap<i32, Vec<usize>>,
    // map follower_id to fo followee_id
    followees: HashMap<i32, HashSet<i32>>,
    // store ten recent tweet_id indices
    recents: HashMap<i32, BinaryHeap<Reverse<usize>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    const N_MOST_RECENTS: usize = 10;

    pub fn new() -> Self {
        Twitter {
            tweets_store: vec![],
            tweets: HashMap::new(),
            followees: HashMap::new(),
            recents: HashMap::new(),
        }
    }
    
    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets_store.push(tweet_id);
        let tidx = self.tweets_store.len() - 1;
        self.tweets.entry(user_id).or_insert(vec![]).push(tidx);

        let user_recents = self.recents.entry(user_id).or_insert(BinaryHeap::new());
        if user_recents.len() >= Self::N_MOST_RECENTS {
            user_recents.pop();
        }
        user_recents.push(Reverse(tidx));
    }
    
    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut news_feed = BinaryHeap::new();
        if let Some(user_posted) = self.recents.get(&user_id) {
            news_feed.extend(user_posted.iter().cloned());
        }

        if let Some(followees) = self.followees.get(&user_id) {
            for followee_id in followees {
                if let Some(followee_posted) = self.recents.get(&followee_id) {
                    for Reverse(t) in followee_posted {
                        if news_feed.len() >= Self::N_MOST_RECENTS {
                            if let Some(Reverse(top)) = news_feed.peek() {
                                if t > top {
                                    news_feed.pop();
                                    news_feed.push(Reverse(*t));
                                }
                            }
                        } else {
                            news_feed.push(Reverse(*t));
                        }
                    }
                }
            }
        }
        let mut result = vec![0; news_feed.len()];
        let mut i = result.len() - 1;
        while let Some(Reverse(idx)) = news_feed.pop() {
            result[i] = self.tweets_store[idx];
            i -= 1;
        }
        result
    }
    
    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.followees.entry(follower_id).or_insert(HashSet::new()).insert(followee_id);
    }
    
    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followees) = self.followees.get_mut(&follower_id) {
            followees.remove(&followee_id);
        }
    }
}
