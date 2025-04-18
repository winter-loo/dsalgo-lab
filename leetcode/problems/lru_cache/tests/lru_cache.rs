use lru_cache::LRUCache;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(1), 1);    // returns 1
        lru_cache.put(3, 3);                // evicts key 2
        assert_eq!(lru_cache.get(2), -1);   // returns -1 (not found)
        lru_cache.put(4, 4);                // evicts key 1
        assert_eq!(lru_cache.get(1), -1);   // returns -1 (not found)
        assert_eq!(lru_cache.get(3), 3);    // returns 3
        assert_eq!(lru_cache.get(4), 4);    // returns 4
    }

    #[test]
    fn test_capacity_one() {
        let mut lru_cache = LRUCache::new(1);
        lru_cache.put(1, 10);
        assert_eq!(lru_cache.get(1), 10);
        lru_cache.put(2, 20); // Evicts 1
        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(2), 20);
        lru_cache.put(3, 30); // Evicts 2
        assert_eq!(lru_cache.get(2), -1);
        assert_eq!(lru_cache.get(3), 30);
    }

    #[test]
    fn test_update_existing_key() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(1), 1);
        lru_cache.put(1, 10); // Update value for key 1
        assert_eq!(lru_cache.get(1), 10);
        assert_eq!(lru_cache.get(2), 2); // Key 2 should still be there
    }

    #[test]
    fn test_get_makes_key_most_recent() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(1), 1); // Access key 1, making it MRU
        lru_cache.put(3, 3); // Should evict key 2 (the LRU)
        assert_eq!(lru_cache.get(2), -1);
        assert_eq!(lru_cache.get(1), 1); // Key 1 should still be present
        assert_eq!(lru_cache.get(3), 3);
    }

    #[test]
    fn test_put_makes_key_most_recent() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        lru_cache.put(1, 10); // Update key 1, should make it MRU
        lru_cache.put(3, 3); // Should evict key 2 (the LRU)
        assert_eq!(lru_cache.get(2), -1);
        assert_eq!(lru_cache.get(1), 10); // Key 1 should still be present
        assert_eq!(lru_cache.get(3), 3);
    }

    #[test]
    fn test_get_non_existent() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        assert_eq!(lru_cache.get(2), -1);
    }
}
