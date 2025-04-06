use valid_palindrome::is_palindrome;

fn main() {
    // Example 1
    let s1 = "A man, a plan, a canal: Panama".to_string();
    println!("Example 1:");
    println!("Input: \"{}\"", s1);
    println!("Output: {}", is_palindrome(s1));
    println!("Expected: true");
    
    // Example 2
    let s2 = "race a car".to_string();
    println!("\nExample 2:");
    println!("Input: \"{}\"", s2);
    println!("Output: {}", is_palindrome(s2));
    println!("Expected: false");
    
    // Example 3
    let s3 = " ".to_string();
    println!("\nExample 3:");
    println!("Input: \"{}\"", s3);
    println!("Output: {}", is_palindrome(s3));
    println!("Expected: true");
} 