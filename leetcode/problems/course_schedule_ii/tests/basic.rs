use course_schedule_ii::Solution;

#[test]
fn test_example_1() {
    // Input: numCourses = 2, prerequisites = [[1,0]]
    // Output: [0,1]
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];

    let result = Solution::find_order(num_courses, prerequisites);
    assert_eq!(result, vec![0, 1]);
}

#[test]
fn test_example_2() {
    // Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
    // Output: [0,2,1,3] or [0,1,2,3]
    let num_courses = 4;
    let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];

    let result = Solution::find_order(num_courses, prerequisites.clone());

    // Check if the result is a valid topological sort
    assert!(is_valid_order(&result, num_courses, &prerequisites));
}

#[test]
fn test_example_3() {
    // Input: numCourses = 1, prerequisites = []
    // Output: [0]
    let num_courses = 1;
    let prerequisites: Vec<Vec<i32>> = vec![];

    let result = Solution::find_order(num_courses, prerequisites);
    assert_eq!(result, vec![0]);
}

#[test]
fn test_cycle() {
    // Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
    // Output: [] (impossible due to cycle)
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0], vec![0, 1]];

    let result = Solution::find_order(num_courses, prerequisites);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_no_prerequisites() {
    // If there are no prerequisites, any order is valid
    let num_courses = 3;
    let prerequisites: Vec<Vec<i32>> = vec![];

    let result = Solution::find_order(num_courses, prerequisites);
    assert_eq!(result.len(), 3);

    // Check that all courses are included
    let mut courses = vec![false; 3];
    for &course in &result {
        courses[course as usize] = true;
    }
    assert!(courses.iter().all(|&present| present));
}

// Helper function to check if the order is valid
fn is_valid_order(order: &[i32], num_courses: i32, prerequisites: &[Vec<i32>]) -> bool {
    if order.len() != num_courses as usize {
        return false;
    }

    // Check that all courses are included
    let mut courses = vec![false; num_courses as usize];
    for &course in order {
        if course < 0 || course >= num_courses || courses[course as usize] {
            return false;
        }
        courses[course as usize] = true;
    }

    // Check that prerequisites are satisfied
    let mut course_positions = vec![0; num_courses as usize];
    for (pos, &course) in order.iter().enumerate() {
        course_positions[course as usize] = pos;
    }

    for prereq in prerequisites {
        let course = prereq[0] as usize;
        let prerequisite = prereq[1] as usize;

        // The prerequisite must come before the course
        if course_positions[prerequisite] >= course_positions[course] {
            return false;
        }
    }

    true
}
