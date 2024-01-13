[leetcode chall](https://leetcode.com/problems/count-complete-tree-nodes/description/)

# Intuition
depth first search using recursivity

# Approach
left then right search (forgot to check if not null the first time)

# Complexity

    Time complexity: 90ms
    Space complexity: 70.60 MB

# Code
```ts
/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

function countNodes(root: TreeNode | null): number {
    if (!root) return 0;
    let left = countNodes(root.left);
    let right = countNodes(root.right);
    
    return left + right + 1;
};
```