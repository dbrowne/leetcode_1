/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/symmetric-tree
//
// Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
//
//
//
// Example 1:
//
//
// Input: root = [1,2,2,3,4,4,3]
// Output: true
// Example 2:
//
//
// Input: root = [1,2,2,null,3,null,3]
// Output: false
//
//
// Constraints:
//
// The number of nodes in the tree is in the range [1, 1000].
// -100 <= Node.val <= 100
//
//
// Follow up: Could you solve it both recursively and iteratively?

pub mod a101 {
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

    use std::rc::Rc;
    use std::cell::RefCell;

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn helper(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            // we only need the references rather then the ownership
            // , so we write &Option<...> in the function signature
            match (left, right) {
                (None, None) => true,
                (None, _) | (_, None) => false,
                (Some(l), Some(r)) => {
                    let (l, r) = (l.borrow(), r.borrow());
                    l.val == r.val && helper(&l.left, &r.right) && helper(&l.right, &r.left)
                }
            }
        }
        helper(&root, &root)
    }

    type Node = Option<Rc<RefCell<TreeNode>>>;
    pub fn is_symmetric_faster(root: Node) -> bool {
        fn check(l: Node, r: Node) -> bool {
            match (l,r) {
                (Some(a),Some(b)) => {
                    let mut _l = a.borrow_mut();
                    let mut _r = b.borrow_mut();
                    _l.val == _r.val && check(_l.left.take(),_r.right.take()) && check(_l.right.take(),_r.left.take())
                },
                (None,None) => true,
                _ => false
            }
        }
        match root {
            Some(r) => {
                let mut _r = r.borrow_mut();
                check(_r.left.take(),_r.right.take())
            },
            _ => true
        }
    }
}

#[cfg(test)]
mod  test{
    use crate::leet75_L2::a_00101_symmetric_tree::a101::is_symmetric;

    #[test]
    fn t0(){
        assert_eq!(true,is_symmetric(None) );
    }
}