use trapping_rain_water::Solution;

#[test]
fn test_example_1() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    assert_eq!(Solution::trap(height), 6);
}

#[test]
fn test_example_2() {
    // -7 + 4 * 4 = 9
    let height = vec![4, 2, 0, 3, 2, 5];
    assert_eq!(Solution::trap(height), 9);
}

#[test]
fn test_empty_array() {
    let height = vec![];
    assert_eq!(Solution::trap(height), 0);
}

#[test]
fn test_single_element() {
    let height = vec![5];
    assert_eq!(Solution::trap(height), 0);
}

#[test]
fn test_no_trap() {
    let height = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::trap(height), 0);
}

#[test]
fn test_decreasing_heights() {
    let height = vec![5, 4, 3, 2, 1];
    assert_eq!(Solution::trap(height), 0);
}

#[test]
fn test_plateau() {
    let height = vec![3, 3, 3, 3];
    assert_eq!(Solution::trap(height), 0);
}

#[test]
fn test_valley() {
    // -14 + 5 * 5
    let height = vec![5, 4, 1, 2, 3, 4, 5];
    assert_eq!(Solution::trap(height), 11);
}

#[test]
fn test_multiple_valleys() {
    let height = vec![5, 2, 1, 2, 1, 5];
    assert_eq!(Solution::trap(height), 14);
}

#[test]
fn test_example_3() {
    let height = vec![2, 4, 2, 1, 3, 2, 5, 4, 2, 3, 4, 5, 6];
    assert_eq!(Solution::trap(height), 15);
}

#[test]
fn test_example_4() {
    let height = vec![5, 5, 1, 7, 1, 1, 5, 2, 7, 6];
    assert_eq!(Solution::trap(height), 23);
}

#[test]
fn test_example_5() {
    let height = vec![
        6, 4, 2, 0, 3, 2, 0, 3, 1, 4, 5, 3, 2, 7, 5, 3, 0, 1, 2, 1, 3, 4, 6, 8, 1, 3,
    ];
    assert_eq!(Solution::trap(height), 83);
}

#[test]
fn test_example_6() {
    // 2 - 0 - 1 = 1 * 9 - 2 = 7 = (2,7)
    // 8 - 2 - 1 = 5 * 8 = 40 - 3 - 2 - 2 - 1 - 4 = 40 - 5 - 3 - 4 = 35 - 7
    // 35 + 7 - 7
    //                0  1  2  3  4  5  6  7  8
    let height = vec![9, 2, 9, 3, 2, 2, 1, 4, 8];
    assert_eq!(Solution::trap(height), 35);
}
