use container_with_most_water::Solution;

#[test]
fn test_example_1() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(Solution::max_area(height), 49);
}

#[test]
fn test_example_2() {
    let height = vec![1, 1];
    assert_eq!(Solution::max_area(height), 1);
}

#[test]
fn test_increasing_heights() {
    let height = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::max_area(height), 6);
}

#[test]
fn test_decreasing_heights() {
    let height = vec![5, 4, 3, 2, 1];
    assert_eq!(Solution::max_area(height), 6);
}

#[test]
fn test_same_heights() {
    let height = vec![5, 5, 5, 5, 5];
    assert_eq!(Solution::max_area(height), 20);
}

#[test]
fn test_mixed_heights() {
    let height = vec![3, 1, 2, 4, 5, 2, 1];
    assert_eq!(Solution::max_area(height), 12);
}

#[test]
fn test_example_3() {
    let height = vec![1, 3, 2, 5, 25, 24, 5];
    println!("{height:?}");
    assert_eq!(Solution::max_area(height), 24);
}
