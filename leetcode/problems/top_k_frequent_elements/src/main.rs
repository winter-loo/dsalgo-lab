use top_k_frequent_elements::top_k_frequent;

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;

    println!("Input: nums = {:?}, k = {}", nums, k);

    // Using heap-based approach
    let result_heap = top_k_frequent(nums.clone(), k);
    println!(
        "Top {} frequent elements (heap approach): {:?}",
        k, result_heap
    );
}
