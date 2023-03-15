/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

/*You are given the root of a binary tree containing digits from 0 to 9 only.

Each root-to-leaf path in the tree represents a number.

For example, the root-to-leaf path 1 -> 2 -> 3 represents the number 123.
Return the total sum of all root-to-leaf numbers. Test cases are generated so that the answer will fit in a 32-bit integer.

A leaf node is a node with no children.



Example 1:


Input: root = [1,2,3]
Output: 25
Explanation:
The root-to-leaf path 1->2 represents the number 12.
The root-to-leaf path 1->3 represents the number 13.
Therefore, sum = 12 + 13 = 25.
Example 2:


Input: root = [4,9,0,5,1]
Output: 1026
Explanation:
The root-to-leaf path 4->9->5 represents the number 495.
The root-to-leaf path 4->9->1 represents the number 491.
The root-to-leaf path 4->0 represents the number 40.
Therefore, sum = 495 + 491 + 40 = 1026.*/

pub mod a129 {
    struct Solution {}

    // Definition for a binary tree node.
    use std::rc::Rc;
    use std::cell::RefCell;

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

    impl Solution {
        pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

            match root {
                Some(root) =>Self::gen_vals(root, 0),
                None => 0,
            }

        }

        fn gen_vals(root: Rc<RefCell<TreeNode>>, number: i32) -> i32 {
            let root = root.borrow();
            let number = number * 10 + root.val;

            match (&root.left, &root.right) {
                (None, None) => number,
                (None, Some(right)) => Self::gen_vals(right.clone(), number),
                (Some(left), None) => Self::gen_vals(left.clone(), number),
                (Some(left), Some(right)) => {
                    Self::gen_vals(right.clone(), number)
                        + Self::gen_vals(left.clone(), number)
                }
            }
        }
    }
}
