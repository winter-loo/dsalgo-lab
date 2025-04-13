use min_stack::MinStack;

#[test]
fn test_example() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);

    assert_eq!(min_stack.get_min(), -3); // return -3
    min_stack.pop();
    assert_eq!(min_stack.top(), 0); // return 0
    assert_eq!(min_stack.get_min(), -2); // return -2
}

#[test]
fn test_single_element() {
    let mut min_stack = MinStack::new();
    min_stack.push(42);

    assert_eq!(min_stack.top(), 42);
    assert_eq!(min_stack.get_min(), 42);
    min_stack.pop();
}

#[test]
fn test_duplicate_values() {
    let mut min_stack = MinStack::new();
    min_stack.push(1);
    min_stack.push(1);
    min_stack.push(1);

    assert_eq!(min_stack.get_min(), 1);
    min_stack.pop();
    assert_eq!(min_stack.get_min(), 1);
}

#[test]
fn test_ascending_values() {
    let mut min_stack = MinStack::new();
    min_stack.push(5);
    min_stack.push(7);
    min_stack.push(9);

    assert_eq!(min_stack.get_min(), 5);
    min_stack.pop();
    assert_eq!(min_stack.get_min(), 5);
    min_stack.pop();
    assert_eq!(min_stack.get_min(), 5);
}

#[test]
fn test_descending_values() {
    let mut min_stack = MinStack::new();
    min_stack.push(9);
    min_stack.push(6);
    min_stack.push(7);
    min_stack.push(5);

    assert_eq!(min_stack.get_min(), 5);
    min_stack.pop();
    assert_eq!(min_stack.get_min(), 6);
    min_stack.pop();
    assert_eq!(min_stack.get_min(), 6);
    min_stack.pop();
    assert_eq!(min_stack.get_min(), 9);
}
