/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// https://leetcode.com/problems/path-sum-iii/
// Given the root of a binary tree and an integer targetSum, return the number of paths where the sum of the values along the path equals targetSum.
//
// The path does not need to start or end at the root or a leaf, but it must go downwards (i.e., traveling only from parent nodes to child nodes).
//
//
//
// Example 1:
//
//
// Input: root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
// Output: 3
// Explanation: The paths that sum to 8 are shown.
// Example 2:
//
// Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
// Output: 3
//
//
// Constraints:
//
// The number of nodes in the tree is in the range [0, 1000].
// -109 <= Node.val <= 109
// -1000 <= targetSum <= 1000

pub mod a437 {
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

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        recurse(&root, target_sum, vec![].as_mut())
    }

    pub fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32, vec: &mut Vec<i32>) -> i32 {
        return if let Some(rc_node) = root {
            let node = rc_node.borrow();
            let mut count = 0;
            vec.push(0);
            vec.iter_mut().for_each(|x| {
                *x = *x + node.val;
                if *x == target_sum {
                    count += 1;
                }
            });
            count = count + recurse(&node.left, target_sum, vec) + recurse(&node.right, target_sum, vec);
            vec.pop();
            vec.iter_mut().for_each(|x| *x = *x - node.val);
            count
        } else {
            0
        };
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn t_0001() {}
}