use valid_palindrome::is_palindrome;

/// Basic test cases from the problem examples
#[test]
fn test_basic_examples() {
    assert_eq!(is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
    assert_eq!(is_palindrome("race a car".to_string()), false);
    assert_eq!(is_palindrome(" ".to_string()), true);
}

/// Test cases for empty and single character strings
#[test]
fn test_empty_and_single_char() {
    // Empty string
    assert_eq!(is_palindrome("".to_string()), true);
    
    // Single character strings
    assert_eq!(is_palindrome("a".to_string()), true);
    assert_eq!(is_palindrome("A".to_string()), true);
    assert_eq!(is_palindrome("5".to_string()), true);
    assert_eq!(is_palindrome("!".to_string()), true); // After filtering, this is empty
}

/// Test cases for strings with only non-alphanumeric characters
#[test]
fn test_non_alphanumeric() {
    assert_eq!(is_palindrome("!@#$%^&*()".to_string()), true); // Empty after filtering
    assert_eq!(is_palindrome(".,;:!@#$%^&*()_+-=".to_string()), true);
    assert_eq!(is_palindrome("  ".to_string()), true);
    assert_eq!(is_palindrome("\t\n ".to_string()), true);
}

/// Test cases for case sensitivity
#[test]
fn test_case_sensitivity() {
    // Mixed case palindromes
    assert_eq!(is_palindrome("Aa".to_string()), true);
    assert_eq!(is_palindrome("aBbA".to_string()), true);
    assert_eq!(is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
    
    // Mixed case non-palindromes
    assert_eq!(is_palindrome("AbC".to_string()), false);
}

/// Test cases for numeric values
#[test]
fn test_numeric_values() {
    // Numeric palindromes
    assert_eq!(is_palindrome("121".to_string()), true);
    assert_eq!(is_palindrome("12321".to_string()), true);
    
    // Numeric non-palindromes
    assert_eq!(is_palindrome("123".to_string()), false);
    
    // Mixed alphanumeric palindromes
    assert_eq!(is_palindrome("a1b2b1a".to_string()), true);
    assert_eq!(is_palindrome("1a2b2a1".to_string()), true);
    
    // Mixed alphanumeric non-palindromes
    assert_eq!(is_palindrome("a1b2c3".to_string()), false);
}

/// Test cases with punctuation and whitespace
#[test]
fn test_punctuation_and_whitespace() {
    // Palindromes with punctuation and whitespace
    assert_eq!(is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
    assert_eq!(is_palindrome("Madam, I'm Adam.".to_string()), true);
    assert_eq!(is_palindrome("Was it a car or a cat I saw?".to_string()), true);
    assert_eq!(is_palindrome("No 'x' in Nixon".to_string()), true);
    
    // Non-palindromes with punctuation and whitespace
    assert_eq!(is_palindrome("Hello, world!".to_string()), false);
    assert_eq!(is_palindrome("This is not a palindrome.".to_string()), false);
}

/// Test cases with sequential and repetitive patterns
#[test]
fn test_sequential_patterns() {
    // Repeating characters
    assert_eq!(is_palindrome("aaa".to_string()), true);
    assert_eq!(is_palindrome("aaabbaaa".to_string()), true);
    
    // Sequential increasing then decreasing
    assert_eq!(is_palindrome("abcba".to_string()), true);
    assert_eq!(is_palindrome("1234321".to_string()), true);
    
    // Sequential increasing only (not palindromes)
    assert_eq!(is_palindrome("abcdef".to_string()), false);
    assert_eq!(is_palindrome("123456".to_string()), false);
    
    // Almost palindromes (off by one character)
    assert_eq!(is_palindrome("abcdba".to_string()), false); // Missing middle 'c'
    
    // This string contains a space in the middle, making it "abcde edcba"
    // After removing spaces, it becomes "abcdeedcba" which IS a palindrome
    let with_space = "abcde edcba".to_string();
    assert_eq!(is_palindrome(with_space), true); 
}

/// Test cases with complex character combinations
#[test]
fn test_complex_combinations() {
    // Complex but valid palindromes
    assert_eq!(is_palindrome("A Santa at NASA".to_string()), true);
    assert_eq!(is_palindrome("No lemon, no melon".to_string()), true);
    assert_eq!(is_palindrome("Never odd or even".to_string()), true);
    
    // Long complex palindromes
    assert_eq!(is_palindrome("Doc, note: I dissent. A fast never prevents a fatness. I diet on cod.".to_string()), true);
    assert_eq!(is_palindrome("T. Eliot, top bard, notes putrid tang emanating, is sad. I'd assign it a name: gnat dirt upset on drab pot toilet.".to_string()), true);
    
    // Complex non-palindromes
    assert_eq!(is_palindrome("This is a test string that is definitely not a palindrome".to_string()), false);
}

/// Test cases with Unicode characters (beyond ASCII)
#[test]
fn test_unicode_characters() {
    // Note: The problem specifies only printable ASCII, but if the implementation handles Unicode:
    // Since our solution uses is_ascii_alphanumeric(), non-ASCII chars are skipped
    assert_eq!(is_palindrome("Ü".to_string()), true); // Treated as empty after filtering
    assert_eq!(is_palindrome("ÜÜ".to_string()), true); // Treated as empty after filtering
    assert_eq!(is_palindrome("ÜüÜ".to_string()), true); // Treated as empty after filtering
    assert_eq!(is_palindrome("Üü".to_string()), true); // Treated as empty after filtering
}

/// Test cases with valid palindromes of various lengths
#[test]
fn test_valid_palindrome_lengths() {
    // Generate palindromes of different lengths
    for length in 1..20 {
        // Even length palindrome
        let even_palindrome = "a".repeat(length) + &"b".repeat(length);
        assert_eq!(is_palindrome(even_palindrome.clone() + &even_palindrome.chars().rev().collect::<String>()), true);
        
        // Odd length palindrome
        let odd_palindrome = "a".repeat(length) + "c" + &"a".repeat(length);
        assert_eq!(is_palindrome(odd_palindrome), true);
    }
}

/// Test cases with systematically created near-palindromes
#[test]
fn test_near_palindromes() {
    // Base palindrome
    let base = "abcdefgfedcba".to_string();
    let result1 = is_palindrome(base.clone());
    println!("Test 1: Base string '{}' is a palindrome: {}", base, result1);
    assert_eq!(result1, true);
    
    // Change first character
    let mut near1 = base.clone();
    near1.replace_range(0..1, "x");
    let result2 = is_palindrome(near1.clone());
    println!("Test 2: String with first char changed '{}' is a palindrome: {}", near1, result2);
    assert_eq!(result2, false);
    
    // Change last character
    let mut near2 = base.clone();
    near2.replace_range(base.len()-1..base.len(), "x");
    let result3 = is_palindrome(near2.clone());
    println!("Test 3: String with last char changed '{}' is a palindrome: {}", near2, result3);
    assert_eq!(result3, false);
    
    // Change middle character
    // Note: Changing the middle character 'g' to 'x' still results in a valid palindrome
    // because the palindrome property only depends on character pairings from outside in
    let mut near3 = base.clone();
    near3.replace_range(base.len()/2..base.len()/2+1, "x");
    let result4 = is_palindrome(near3.clone());
    println!("Test 4: String with middle char changed '{}' is a palindrome: {}", near3, result4);
    assert_eq!(result4, true); // Corrected to expect true
    
    // Test a new palindrome
    let new_palindrome = "abcdeffedcba".to_string();
    let result5 = is_palindrome(new_palindrome.clone());
    println!("Test 5: String '{}' is a palindrome: {}", new_palindrome, result5);
    assert_eq!(result5, true);
}

/// Test cases with boundary conditions
#[test]
fn test_boundary_conditions() {
    // Very long palindrome (within constraint limits)
    let long_palindrome = "a".repeat(100_000) + &"a".repeat(100_000);
    assert_eq!(is_palindrome(long_palindrome), true);
    
    // Mixed long palindrome
    let mixed_long = "a".repeat(50_000) + &"b".repeat(50_000) + &"b".repeat(50_000) + &"a".repeat(50_000);
    assert_eq!(is_palindrome(mixed_long), true);
} 