/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/kth-smallest-element-in-a-bst/
// Given the root of a binary search tree, and an integer k, return the kth smallest value (1-indexed) of all the values of the nodes in the tree.
//
//
//
// Example 1:
//
//
// Input: root = [3,1,4,null,2], k = 1
// Output: 1
// Example 2:
//
//
// Input: root = [5,3,6,2,4,null,null,1], k = 3
// Output: 3
//
//
// Constraints:
//
// The number of nodes in the tree is n.
// 1 <= k <= n <= 104
// 0 <= Node.val <= 104
//
//
// Follow up: If the BST is modified often (i.e., we can do insert and delete operations) and you need to find the kth smallest frequently, how would you optimize?

pub mod a208 {
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

    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        inorder(root, k, 0).1.unwrap()
    }

    fn inorder(node_option: Option<Rc<RefCell<TreeNode>>>, k: i32, acc: i32) -> (i32, Option<i32>) {
        if let Some(node_rc) = node_option {
            let mut node_ref = node_rc.borrow_mut();
            let (acc, r) = inorder(node_ref.left.take(), k, acc);
            if acc == k {
                return (acc, r);
            }
            let acc = acc + 1;
            if acc == k {
                return (acc, Some(node_ref.val));
            }
            inorder(node_ref.right.take(), k, acc)
        } else {
            (acc, None)
        }
    }

    pub fn kth_smallest_iterative(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut k = k;
        let mut stack = vec![];
        let mut curr = root;

        loop {
            while let Some(node) = curr {
                curr = node.borrow_mut().left.take();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                k -= 1;
                if k == 0 {
                    return node.borrow().val;
                }
                curr = node.borrow_mut().right.take();
            } else {
                panic!("BST too small");
            }
        }
    }
}
#[cfg(test)]
mod test {

    #[test]
    fn t_0001() {
        assert_eq!(0, 0)
    }
}

