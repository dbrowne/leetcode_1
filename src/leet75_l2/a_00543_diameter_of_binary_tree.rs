/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// https://leetcode.com/problems/diameter-of-binary-tree/
// Given the root of a binary tree, return the length of the diameter of the tree.
//
// The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
//
// The length of a path between two nodes is represented by the number of edges between them.
//
//
//
// Example 1:
//
//
// Input: root = [1,2,3,4,5]
// Output: 3
// Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].
// Example 2:
//
// Input: root = [1,2]
// Output: 1
//
//
// Constraints:
//
// The number of nodes in the tree is in the range [1, 104].
// -100 <= Node.val <= 100


pub mod a542 {
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

    use std::rc::Rc;
    use std::cell::RefCell;

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter: i32 = 0;
        dfs(&root, &mut diameter);
        return diameter;
    }

    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
        match root {
            Some(root) => {
                let root = root.borrow();
                let left: i32 = dfs(&root.left, diameter);
                let right: i32 = dfs(&root.right, diameter);
                *diameter = std::cmp::max(*diameter, left + right);
                std::cmp::max(left, right) + 1
            }
            None => {
                return 0;
            }
        }
    }


    type NodeRef = Option<Rc<RefCell<TreeNode>>>;

    pub fn diameter_of_binary_tree_faster(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn get_diameter(root: &NodeRef, d: &mut i32) -> i32 {
            match root {
                Some(root_rc) => {
                    let node = &*root_rc.borrow();
                    let TreeNode { left, right, .. } = node;
                    let left_d = get_diameter(&left, d);
                    let right_d = get_diameter(&right, d);
                    *d = (left_d + right_d).max(*d);
                    1 + left_d.max(right_d)
                }
                None => {
                    return 0;
                }
            }
        }

        let mut d = 0;
        get_diameter(&root, &mut d);
        d
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn t_001() {
        assert_eq!(0, -1);
    }
}