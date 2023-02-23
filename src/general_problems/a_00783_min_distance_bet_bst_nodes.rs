/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given the root of a Binary Search Tree (BST), return the minimum difference between the values of any two different nodes in the tree.
//
//
//
// Example 1:
//
//
// Input: root = [4,2,6,1,3]
// Output: 1
// Example 2:
//
//
// Input: root = [1,0,48,null,null,12,49]
// Output: 1
//
//
// Constraints:
//
// The number of nodes in the tree is in the range [2, 100].
// 0 <= Node.val <= 105
//
//
// Note: This question is the same as 530: https://leetcode.com/problems/minimum-absolute-difference-in-bst/

mod  a783{
    use std::rc::Rc;
    use std::cell::RefCell;

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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans:i32 = 0;
        let  mut stk:Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        stk.push(root);
        let  mut vals:Vec<i32> = vec![];
        while let Some (node) = stk.pop(){
            match node {
                Some(visited) => {
                    let  tmp = visited.as_ref().borrow();
                    vals.push(tmp.val);
                    stk.push(tmp.left.clone());
                    stk.push(tmp.right.clone());
                }
                None =>{}
            }

        }

        vals.sort_unstable();
        ans = vals.windows(2).map(|tt| tt[1] - tt[0]).min().unwrap_or(-1);
        ans

    }


}