/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/kth-smallest-element-in-a-bst/
//
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

pub  mod  a230{
    struct Solution{}

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
    use std::rc::Rc;
    use std::cell::RefCell;
    impl Solution {
        pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
            Self::inorder(root, k, 0).1.unwrap()
        }

        fn inorder(node_option: Option<Rc<RefCell<TreeNode>>>, k: i32, acc: i32) -> (i32, Option<i32>) {
            if let Some(node_rc) = node_option {
                let mut node_ref = node_rc.borrow_mut();
                let (acc, r) = Self::inorder(node_ref.left.take(), k, acc);
                if acc == k {
                    return (acc, r);
                }
                let acc = acc + 1;
                if acc == k {
                    return (acc, Some(node_ref.val));
                }
                Self::inorder(node_ref.right.take(), k, acc)
            } else {
                (acc, None)
            }
        }
    }


    // BEST!

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = vec![];
        inorder(&root, &mut stack);

        stack[k as usize - 1]
    }

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>) -> () {
        if let Some(node_rc) = node {
            let _node = node_rc.borrow();

            inorder(&_node.left, stack);
            stack.push(_node.val);
            inorder(&_node.right, stack);
        }
    }

    /// Alternative
    ///
    ///

    pub fn kth_smallest_1(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        let mut node = root;
        let mut k = k;

        while node.is_some() || ! stack.is_empty() {
            while let Some(n) = node {
                node = n.borrow_mut().left.take();
                stack.push(Some(n));
            }

            if let Some(Some(n)) = stack.pop() {
                k = k - 1;
                if k == 0 {
                    return n.borrow().val;
                }
                node = n.borrow_mut().right.take();
            }
        }
        unreachable!()
    }


}

#[cfg(test)]
mod  test {
    use crate::leet75_L2::a_208_kth_smallest_element_in_bst::a208::kth_smallest;

    #[test]
    fn t_000(){
        assert_eq!(0,kth_smallest(None,1) );
    }
}