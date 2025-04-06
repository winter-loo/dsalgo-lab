use product_of_array_except_self::product_except_self;

fn main() {
    println!("Product of Array Except Self");
    
    let nums = vec![1, 2, 3, 4];
    println!("Input: {:?}", nums);
    println!("Expected Output: [24, 12, 8, 6]");
    println!("Your Output: {:?}", product_except_self(nums));
}
