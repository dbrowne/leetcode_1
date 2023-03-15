/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

/*Given the root of a binary tree, determine if it is a complete binary tree.

In a complete binary tree, every level, except possibly the last, is completely filled, and all nodes in the last level are as far left as possible. It can have between 1 and 2h nodes inclusive at the last level h.



Example 1:


Input: root = [1,2,3,4,5,6]
Output: true
Explanation: Every level before the last is full (ie. levels with node-values {1} and {2, 3}), and all nodes in the last level ({4, 5, 6}) are as far left as possible.
Example 2:


Input: root = [1,2,3,4,5,null,7]
Output: false
Explanation: The node with value 7 isn't as far left as possible.


Constraints:

The number of nodes in the tree is in the range [1, 100].
1 <= Node.val <= 1000*/


pub  mod  a958{
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
    struct Solution{}

    use std::rc::Rc;
    use std::cell::RefCell;
    impl Solution {

        pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            let mut visited: Vec<bool> = Vec::new();
            let mut complete: Vec<bool> = Vec::new();
            if !Self::traverse(root, 0, true, &mut visited, &mut complete) {
                return false;
            }
            for i in (0..complete.len()-2).rev() {
                if !complete[i] {
                    return false;
                }
            }
            true
        }


        fn traverse(root: Option<Rc<RefCell<TreeNode>>>, level: usize, right: bool, visited: &mut Vec<bool>, complete: &mut Vec<bool>) -> bool {
            if visited.len() == level {
                visited.push(false);
            }
            if complete.len() == level {
                complete.push(false);
            }
            match root {
                Some(node) => {
                    if right {
                        complete[level] = true;
                    }
                    visited[level] = true;
                    if !Self::traverse(node.borrow().right.clone(), level + 1, right, visited, complete) {
                        return false;
                    }
                    Self::traverse(node.borrow().left.clone(), level + 1, false, visited, complete)
                },
                None => !visited[level]
            }
        }
    }



}