use time_based_key_value_store::TimeMap;

#[test]
fn test_example_1() {
    let mut time_map = TimeMap::new();
    time_map.set("foo".to_string(), "bar".to_string(), 1);
    assert_eq!(time_map.get("foo".to_string(), 1), "bar".to_string());
    assert_eq!(time_map.get("foo".to_string(), 3), "bar".to_string());
    time_map.set("foo".to_string(), "bar2".to_string(), 4);
    assert_eq!(time_map.get("foo".to_string(), 4), "bar2".to_string());
    assert_eq!(time_map.get("foo".to_string(), 5), "bar2".to_string());
}

#[test]
fn test_nonexistent_key() {
    let time_map = TimeMap::new();
    assert_eq!(time_map.get("nonexistent".to_string(), 1), "".to_string());
}

#[test]
fn test_timestamp_too_early() {
    let mut time_map = TimeMap::new();
    time_map.set("key".to_string(), "value".to_string(), 5);
    assert_eq!(time_map.get("key".to_string(), 3), "".to_string());
}

#[test]
fn test_multiple_values_same_key() {
    let mut time_map = TimeMap::new();
    time_map.set("key".to_string(), "value1".to_string(), 1);
    time_map.set("key".to_string(), "value2".to_string(), 2);
    time_map.set("key".to_string(), "value3".to_string(), 3);
    
    assert_eq!(time_map.get("key".to_string(), 1), "value1".to_string());
    assert_eq!(time_map.get("key".to_string(), 2), "value2".to_string());
    assert_eq!(time_map.get("key".to_string(), 3), "value3".to_string());
    assert_eq!(time_map.get("key".to_string(), 4), "value3".to_string());
}

#[test]
fn test_multiple_keys() {
    let mut time_map = TimeMap::new();
    time_map.set("key1".to_string(), "value1".to_string(), 1);
    time_map.set("key2".to_string(), "value2".to_string(), 1);
    
    assert_eq!(time_map.get("key1".to_string(), 1), "value1".to_string());
    assert_eq!(time_map.get("key2".to_string(), 1), "value2".to_string());
    assert_eq!(time_map.get("key1".to_string(), 2), "value1".to_string());
}

#[test]
fn test_exact_timestamp_match() {
    let mut time_map = TimeMap::new();
    time_map.set("key".to_string(), "value1".to_string(), 10);
    time_map.set("key".to_string(), "value2".to_string(), 20);
    
    assert_eq!(time_map.get("key".to_string(), 10), "value1".to_string());
    assert_eq!(time_map.get("key".to_string(), 15), "value1".to_string());
    assert_eq!(time_map.get("key".to_string(), 20), "value2".to_string());
    assert_eq!(time_map.get("key".to_string(), 25), "value2".to_string());
}
