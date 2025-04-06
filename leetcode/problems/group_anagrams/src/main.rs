use group_anagrams::group_anagrams;

fn main() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    
    let result = group_anagrams(strs);
    
    println!("Grouped Anagrams:");
    for (i, group) in result.iter().enumerate() {
        println!("Group {}: {:?}", i + 1, group);
    }
} 