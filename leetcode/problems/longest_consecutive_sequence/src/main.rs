use longest_consecutive_sequence::longest_consecutive;

fn main() {
    // Example 1
    let nums1 = vec![100, 4, 200, 1, 3, 2];
    println!("Example 1:");
    println!("Input: {:?}", nums1);
    println!("Output: {}", longest_consecutive(nums1));
    println!("Expected: 4");
    
    // Example 2
    let nums2 = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    println!("\nExample 2:");
    println!("Input: {:?}", nums2);
    println!("Output: {}", longest_consecutive(nums2));
    println!("Expected: 9");
} 