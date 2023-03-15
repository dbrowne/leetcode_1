pub mod tentwosix {
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

    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                return 0;
            }
            None => {
                return -1;
            }
        }
    }
    pub fn helper(node: TreeNode, curMax: i32, curMin: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::general_problems::a_1026_max_diff_bet_node_and_ancr::tentwosix::{
        max_ancestor_diff, TreeNode,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_0() {
        let x = TreeNode::new(1);
        assert_eq!(0, max_ancestor_diff(Some(Rc::new(RefCell::new(x)))));
    }
}
