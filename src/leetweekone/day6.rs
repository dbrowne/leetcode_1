pub mod day_six {
    use std::cell::RefCell;
    use std::rc::Rc;

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

    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut answer = vec![];

        let mut root_vec = vec![root];
        let mut child_vec = vec![];

        let mut swap = true;

        loop {
            if root_vec.is_empty() && child_vec.is_empty() {
                break;
            }

            let (parent, child) = if swap {
                (&mut root_vec, &mut child_vec)
            } else {
                (&mut child_vec, &mut root_vec)
            };

            let map: Vec<i32> = parent
                .iter()
                .filter_map(|node| {
                    if let Some(node_value) = node {
                        let node_value_borrowed = node_value.borrow();
                        child.push(node_value_borrowed.left.clone());
                        child.push(node_value_borrowed.right.clone());
                        Some(node_value_borrowed.val)
                    } else {
                        None
                    }
                })
                .collect();
            if !map.is_empty() {
                answer.push(map)
            }
            parent.clear();
            swap = !swap;
        }
        answer
    }
}

#[cfg(test)]
mod test {
    // https://github.com/Kilerd/leetcode-rust/blob/master/src/binary_tree_level_order_traversal.rs
    use crate::leetweekone::day6::day_six::{level_order, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_1() {
        let node15 = TreeNode::new(15);
        let node7 = TreeNode::new(7);

        let mut node20 = TreeNode::new(20);
        node20.left = Some(Rc::new(RefCell::new(node15)));
        node20.right = Some(Rc::new(RefCell::new(node7)));

        let node9 = TreeNode::new(9);
        let mut node3 = TreeNode::new(3);
        node3.left = Some(Rc::new(RefCell::new(node20)));
        node3.right = Some(Rc::new(RefCell::new(node9)));

        let ans_vec = vec![vec![3], vec![20, 9], vec![15, 7]];
        assert_eq!(ans_vec, level_order(Some(Rc::new(RefCell::new(node3)))));
    }

    #[test]
    fn test_2() {
        let ans_vec = vec![vec![4]];
        assert_eq!(
            ans_vec,
            level_order(Some(Rc::new(RefCell::new(TreeNode::new(4)))))
        );
    }
}
