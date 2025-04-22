// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Initialize max_sum with the smallest possible value.
        // Node values can be negative, so 0 is not a safe initial value.
        let mut max_sum = i32::MIN;
        // Call the recursive helper function. The return value of dfs isn't
        // directly used here; its purpose is to update max_sum by reference.
        Self::dfs(&root, &mut max_sum);
        // Return the final maximum sum found.
        max_sum
    }

    /// Recursive helper function (DFS).
    ///
    /// Args:
    ///   node_opt: The current node being processed (Option<Rc<RefCell<TreeNode>>>).
    ///   max_sum: A mutable reference to the overall maximum path sum found so far.
    ///
    /// Returns:
    ///   The maximum path sum starting at `node_opt` and extending downwards
    ///   into at most one of its children's subtrees (or 0 if the gain is negative).
    ///   This is the "gain" the current node can offer to its parent.
    ///
    /// Side Effect:
    ///   Updates `max_sum` if a path passing through `node_opt` (potentially
    ///   using both children) yields a sum greater than the current `max_sum`.
    fn dfs(node_opt: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        match node_opt {
            Some(node_rc) => {
                let node = node_rc.borrow();

                // Recursively calculate the maximum gain from the left and right subtrees.
                // Use max(0, ...) to ignore paths with negative sums from subtrees,
                // as we wouldn't extend a path into a negative-sum branch.
                let left_gain = max(0, Self::dfs(&node.left, max_sum));
                let right_gain = max(0, Self::dfs(&node.right, max_sum));

                // Calculate the maximum path sum *passing through* the current node.
                // This path includes the current node's value plus the gains from
                // potentially both left and right branches.
                let path_through_current = node.val + left_gain + right_gain;

                // Update the overall maximum sum if the path through the current node is greater.
                *max_sum = max(*max_sum, path_through_current);

                // Return the maximum gain *ending* at the current node.
                // This path includes the current node's value plus the gain from
                // the *better* of the two children branches (left or right).
                // This is the value needed by the parent node.
                node.val + max(left_gain, right_gain)
            }
            None => {
                // Base case: A null node contributes 0 gain to any path.
                0
            }
        }
    }
}
