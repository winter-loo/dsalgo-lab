pub fn is_palindrome(s: &str) -> bool {
    // Convert to bytes for easier ASCII handling
    let bytes = s.as_bytes();
    let mut left = 0;
    let mut right = bytes.len().saturating_sub(1);
    
    while left < right {
        // Skip non-alphanumeric characters from the left
        while left < bytes.len() && !bytes[left].is_ascii_alphanumeric() {
            left += 1;
        }
        
        // Skip non-alphanumeric characters from the right
        while right > 0 && !bytes[right].is_ascii_alphanumeric() {
            right = right.saturating_sub(1);
        }
        
        // If we've exhausted the string or crossed pointers, it's a palindrome
        if left >= right {
            return true;
        }
        
        // Compare characters (case-insensitive)
        if bytes[left].to_ascii_lowercase() != bytes[right].to_ascii_lowercase() {
            return false;
        }
        
        left += 1;
        right = right.saturating_sub(1);
    }
    
    true
}
