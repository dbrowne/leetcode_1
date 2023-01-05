/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/balanced-binary-tree/
//
// Given a binary tree, determine if it is
// height-balanced
// .
//
//
//
// Example 1:
//
//
// Input: root = [3,9,20,null,null,15,7]
// Output: true
// Example 2:
//
//
// Input: root = [1,2,2,3,3,null,null,4,4]
// Output: false
// Example 3:
//
// Input: root = []
// Output: true
//
//
// Constraints:
//
// The number of nodes in the tree is in the range [0, 5000].
// -104 <= Node.val <= 104

pub  mod a110 {
    use std::rc::Rc;
    use std::cell::RefCell;
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

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut f = true;
        is_b(root.as_ref(), &mut f);
        f
    }

    pub fn is_b(root: Option<&Rc<RefCell<TreeNode>>>, f: &mut bool) -> i32 {
        match root {
            None => return 0,
            Some(node_ref) => {
                let left = is_b(node_ref.borrow_mut().left.as_ref(), f) + 1;
                let right = is_b(node_ref.borrow_mut().right.as_ref(), f) + 1;
                if ((left - right).abs() > 1) {
                    *f = false;
                }
                return std::cmp::max(left, right);
            },
        }
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn t_001(){
        assert_eq!(0,0 );
    }
}