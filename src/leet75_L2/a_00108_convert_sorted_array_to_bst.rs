/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

pub  mod  a108{
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
    use std::collections::VecDeque;

        pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            let len = nums.len();
            let mut stack = VecDeque::new();
            let root = Rc::new(RefCell::new(TreeNode::new(nums[(len-1)/2])));

            stack.push_back((0, len-1, Rc::clone(&root)));

            while let Some((start, end, node)) = stack.pop_back() {
                let mid = start + (end - start) / 2;
                if start < mid {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(nums[start + (mid - 1 - start) / 2])));
                    stack.push_back((start, mid-1, Rc::clone(&new_node)));
                    node.as_ref().borrow_mut().left = Some(new_node);
                }
                if end > mid {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(nums[mid + 1 + (end - mid - 1) / 2])));
                    stack.push_back((mid+1, end, Rc::clone(&new_node)));
                    node.as_ref().borrow_mut().right = Some(new_node);
                }
            }
            Some(root)

        }

}

#[cfg(test)]
mod  test{
    use crate::leet75_L2::a_00108_convert_sorted_array_to_bst::a108::sorted_array_to_bst;

    #[test]
    fn t_001(){
        assert_eq!(None,sorted_array_to_bst(vec![0]) );
    }
}