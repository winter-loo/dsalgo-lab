use reconstruct_itinerary::Solution;

#[test]
fn test_example_1() {
    // Input: tickets = [["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]
    // Output: ["JFK","MUC","LHR","SFO","SJC"]
    let tickets = vec![
        vec!["MUC".to_string(), "LHR".to_string()],
        vec!["JFK".to_string(), "MUC".to_string()],
        vec!["SFO".to_string(), "SJC".to_string()],
        vec!["LHR".to_string(), "SFO".to_string()],
    ];
    
    let expected = vec![
        "JFK".to_string(),
        "MUC".to_string(),
        "LHR".to_string(),
        "SFO".to_string(),
        "SJC".to_string(),
    ];
    
    assert_eq!(Solution::find_itinerary(tickets), expected);
}

#[test]
fn test_example_2() {
    // Input: tickets = [["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]]
    // Output: ["JFK","ATL","JFK","SFO","ATL","SFO"]
    let tickets = vec![
        vec!["JFK".to_string(), "SFO".to_string()],
        vec!["JFK".to_string(), "ATL".to_string()],
        vec!["SFO".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "JFK".to_string()],
        vec!["ATL".to_string(), "SFO".to_string()],
    ];
    
    let expected = vec![
        "JFK".to_string(),
        "ATL".to_string(),
        "JFK".to_string(),
        "SFO".to_string(),
        "ATL".to_string(),
        "SFO".to_string(),
    ];
    
    assert_eq!(Solution::find_itinerary(tickets), expected);
}

#[test]
fn test_single_ticket() {
    // Input: tickets = [["JFK","SFO"]]
    // Output: ["JFK","SFO"]
    let tickets = vec![
        vec!["JFK".to_string(), "SFO".to_string()],
    ];
    
    let expected = vec![
        "JFK".to_string(),
        "SFO".to_string(),
    ];
    
    assert_eq!(Solution::find_itinerary(tickets), expected);
}

#[test]
fn test_lexical_order() {
    // Test that the itinerary with the smallest lexical order is returned
    // Input: tickets = [["JFK","KUL"],["JFK","NRT"],["NRT","JFK"]]
    // Output: ["JFK","KUL"] has smaller lexical order than ["JFK","NRT"]
    // So the output should be ["JFK","NRT","JFK","KUL"]
    let tickets = vec![
        vec!["JFK".to_string(), "KUL".to_string()],
        vec!["JFK".to_string(), "NRT".to_string()],
        vec!["NRT".to_string(), "JFK".to_string()],
    ];
    
    let expected = vec![
        "JFK".to_string(),
        "NRT".to_string(),
        "JFK".to_string(),
        "KUL".to_string(),
    ];
    
    assert_eq!(Solution::find_itinerary(tickets), expected);
}
