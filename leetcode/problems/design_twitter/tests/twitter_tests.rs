// Import the struct/functions from your library crate
use design_twitter::Twitter;

#[test]
fn test_example_1() {
    let mut twitter = Twitter::new();
    twitter.post_tweet(1, 5);
    assert_eq!(twitter.get_news_feed(1), vec![5], "Test Case 1 Failed: Initial post");

    twitter.follow(1, 2);
    twitter.post_tweet(2, 6);
    // Feed should contain user 1's tweet and user 2's tweet (since 1 follows 2)
    // Expected order: most recent first
    assert_eq!(twitter.get_news_feed(1), vec![6, 5], "Test Case 1 Failed: After follow and second post");

    twitter.unfollow(1, 2);
    // Feed should only contain user 1's tweet now
    assert_eq!(twitter.get_news_feed(1), vec![5], "Test Case 1 Failed: After unfollow");
}

#[test]
fn test_example_2() {
    let mut twitter = Twitter::new();
    twitter.post_tweet(2, 5);
    twitter.follow(1, 2);
    twitter.follow(1, 2);

    assert_eq!(twitter.get_news_feed(1), vec![5]);
}

#[test]
fn test_example_3() {
    let mut twitter = Twitter::new();
    twitter.post_tweet(1, 5);
    twitter.post_tweet(1, 3);
    twitter.post_tweet(1, 101);
    twitter.post_tweet(1, 13);
    twitter.post_tweet(1, 10);
    twitter.post_tweet(1, 2);
    twitter.post_tweet(1, 94);
    twitter.post_tweet(1, 505);
    twitter.post_tweet(1, 333);
    twitter.post_tweet(1, 22);
    twitter.post_tweet(1, 11);

    assert_eq!(twitter.get_news_feed(1), vec![11,22,333,505,94,2,10,13,101,3]);
}
