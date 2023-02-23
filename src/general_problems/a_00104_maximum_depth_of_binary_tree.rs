/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given the root of a binary tree, return its maximum depth.
//
// A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
//
//
//
// Example 1:
//
//
// Input: root = [3,9,20,null,null,15,7]
// Output: 3
// Example 2:
//
// Input: root = [1,null,2]
// Output: 2
//
//
// Constraints:
//
// The number of nodes in the tree is in the range [0, 104].
// -100 <= Node.val <= 100



pub  mod  a_104 {
    // Definition for a binary tree node.
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
                right: None
            }
        }
    }

    use std::borrow::BorrowMut;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::cmp::max;

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        // if root.is_none() {
        //     return 0;
        // }
        //
        // let mut s = vec![(root.unwrap(), 1)];
        // let mut max_depth = 0;
        // while let Some((rc, depth)) = s.pop() {
        //     max_depth = max_depth.max(depth);
        //
        //     let mut rc_mut = rc.borrow_mut();
        //     if let Some(left) = rc_mut.left.take() { s.push((left, depth + 1)); }
        //     if let Some(right) = rc_mut.right.take() { s.push((right, depth + 1)); }
        // }
        max_depth
    }
}