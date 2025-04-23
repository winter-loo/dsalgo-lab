use find_median_from_data_stream::MedianFinder;

#[test]
fn test_example_1() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(1); // arr = [1]
    median_finder.add_num(2); // arr = [1, 2]
    assert_eq!(median_finder.find_median(), 1.5); // return 1.5 (i.e., (1 + 2) / 2)
    median_finder.add_num(3); // arr = [1, 2, 3]
    println!("median_finder:{median_finder:?}");
    assert_eq!(median_finder.find_median(), 2.0); // return 2.0
}

#[test]
fn test_single_element() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(5);
    assert_eq!(median_finder.find_median(), 5.0);
}

#[test]
fn test_odd_number_of_elements() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(1);
    median_finder.add_num(3);
    median_finder.add_num(5);
    assert_eq!(median_finder.find_median(), 3.0);
}

#[test]
fn test_even_number_of_elements() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(1);
    println!("{median_finder:?}");
    median_finder.add_num(2);
    println!("{median_finder:?}");
    median_finder.add_num(3);
    println!("{median_finder:?}");
    median_finder.add_num(4);
    println!("{median_finder:?}");
    assert_eq!(median_finder.find_median(), 2.5);
}

#[test]
fn test_unsorted_insertion() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(5);
    median_finder.add_num(2);
    median_finder.add_num(1);
    median_finder.add_num(4);
    median_finder.add_num(3);
    assert_eq!(median_finder.find_median(), 3.0);
}

#[test]
fn test_example_2() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(12);
    assert_eq!(median_finder.find_median(), 12.0);
    median_finder.add_num(10);
    // 10,12
    println!("{median_finder:?}");
    assert_eq!(median_finder.find_median(), 11.0);
    median_finder.add_num(13);
    // 10,12,13
    println!("{median_finder:?}");
    assert_eq!(median_finder.find_median(), 12.0);
    median_finder.add_num(11);
    // 10,11,12,13
    println!("{median_finder:?}");
    assert_eq!(median_finder.find_median(), 11.5);
    median_finder.add_num(5);
    // 5,10,11,12,13
    assert_eq!(median_finder.find_median(), 11.0);
    median_finder.add_num(15);
    // 5,10,11,12,13,15
    assert_eq!(median_finder.find_median(), 11.5);
    median_finder.add_num(1);
    // 1,5,10,11,12,13,15
    assert_eq!(median_finder.find_median(), 11.0);
}
