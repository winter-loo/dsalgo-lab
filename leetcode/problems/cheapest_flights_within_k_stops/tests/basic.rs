use cheapest_flights_within_k_stops::Solution;

#[test]
fn test_example_1() {
    // Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1
    // Output: 700
    let n = 4;
    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![2, 0, 100],
        vec![1, 3, 600],
        vec![2, 3, 200],
    ];
    let src = 0;
    let dst = 3;
    let k = 1;
    
    assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 700);
}

#[test]
fn test_example_2() {
    // Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 1
    // Output: 200
    let n = 3;
    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![0, 2, 500],
    ];
    let src = 0;
    let dst = 2;
    let k = 1;
    
    assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 200);
}

#[test]
fn test_example_3() {
    // Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 0
    // Output: 500
    let n = 3;
    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![0, 2, 500],
    ];
    let src = 0;
    let dst = 2;
    let k = 0;
    
    assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 500);
}

#[test]
fn test_no_path() {
    // Input: n = 3, flights = [[0,1,100],[1,2,100]], src = 0, dst = 2, k = 0
    // Output: -1 (No direct path from 0 to 2)
    let n = 3;
    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
    ];
    let src = 0;
    let dst = 2;
    let k = 0;
    
    assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), -1);
}

#[test]
fn test_complex_graph() {
    // Input: n = 5, flights = [[0,1,100],[0,2,500],[1,2,100],[1,3,600],[2,3,200],[2,4,350],[3,4,200]], src = 0, dst = 4, k = 2
    // Output: 700 (Path: 0->1->2->4 or 0->2->4)
    let n = 5;
    let flights = vec![
        vec![0, 1, 100],
        vec![0, 2, 500],
        vec![1, 2, 100],
        vec![1, 3, 600],
        vec![2, 3, 200],
        vec![2, 4, 350],
        vec![3, 4, 200],
    ];
    let src = 0;
    let dst = 4;
    let k = 2;
    
    assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 650);
}
