pub mod day_three {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(l), None) => return Some(l),
            (None, Some(r)) => return Some(r),
            (None, None) => return None,
            (Some(l), Some(r)) => {
                if l.val <= r.val {
                    return Some(Box::new(ListNode {
                        next: merge_two_lists(l.next, Some(r)),
                        val: l.val,
                    }));
                } else {
                    return Some(Box::new(ListNode {
                        next: merge_two_lists(Some(l), r.next),
                        val: r.val,
                    }));
                }
            }
        }
    }

    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut cur = head;
        while cur.is_some() {
            let next = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = pre;
            pre = cur;
            cur = next;
        }
        pre
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn can_merge_1() {
            let input1 = Some(Box::new(ListNode::new(1)));
            let input2 = Some(Box::new(ListNode::new(2)));
            let first_node = ListNode {
                next: Some(Box::new(ListNode::new(2))),
                val: 1,
            };

            let expected = Some(Box::new(first_node));
            assert_eq!(expected, merge_two_lists(input1, input2));
        }

        #[test]
        fn can_merge_2() {
            let input1 = None;
            let input2 = Some(Box::new(ListNode::new(2)));

            let expected = Some(Box::new(ListNode::new(2)));
            assert_eq!(expected, merge_two_lists(input1, input2));
        }

        #[test]
        fn can_merge_3() {
            let input1 = Some(Box::new(ListNode::new(2)));
            let input2 = None;

            let expected = input1.clone();
            assert_eq!(expected, merge_two_lists(input1, input2));
        }

        #[test]
        fn can_merge_4() {
            let input1 = None;
            let input2 = None;

            let expected = None;
            assert_eq!(expected, merge_two_lists(input1, input2));
        }
    }
}
