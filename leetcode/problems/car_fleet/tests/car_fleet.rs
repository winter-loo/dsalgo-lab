use car_fleet::Solution;

#[test]
fn test_example_1() {
    let target = 12;
    let position = vec![10,8,0,5,3];
    let speed = vec![2,4,1,1,3];

    let n = Solution::car_fleet(target, position, speed);
    assert_eq!(n, 3);
}

#[test]
fn test_example_2() {
    let target = 10;
    let position = vec![3];
    let speed = vec![3];

    let n = Solution::car_fleet(target, position, speed);
    assert_eq!(n, 1);
}

#[test]
fn test_example_3() {
    let target = 20;
    let position = vec![6, 2, 17];
    let speed = vec![3, 9 , 2];

    let n = Solution::car_fleet(target, position, speed);
    assert_eq!(n, 2);
}

#[test]
fn test_example_4() {
    let target = 10;
    let position = vec![8, 3, 7, 4, 6, 5];
    let speed = vec![4, 4, 4, 4, 4, 4];

    let n = Solution::car_fleet(target, position, speed);
    assert_eq!(n, 6);
}
