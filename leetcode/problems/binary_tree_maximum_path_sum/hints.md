# Hints for Binary Tree Maximum Path Sum

1. A path in the tree can be viewed as starting from any node, going up to a common ancestor, and then down to another node.

2. For each node, consider it as the highest point (turning point) of a path. The maximum path through this node would be: `left_max_gain + node.val + right_max_gain`.

3. The key is to compute the maximum gain you can get from each subtree. This gain might be negative, in which case it's better to exclude that subtree from the path.

4. Use a recursive approach where each node returns the maximum path sum that can be extended upward (i.e., including at most one child).

5. Keep a global variable to track the maximum path sum seen so far, which can include both children of a node.
