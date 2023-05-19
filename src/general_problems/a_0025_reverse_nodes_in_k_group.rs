/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

pub mod a25 {



    // Definition for singly-linked list.
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode {
                next: None,
                val,
            }
        }
    }

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 1 {
            return head;
        }
        let mut head:Option<Box<ListNode>> = head;
        let mut result:Option<Box<ListNode>> = None;
        let mut result_tail: &mut Option<Box<ListNode>> = &mut result;
        let (mut g_head, mut g_count) = (None, 0);

        while let Some(mut node) = head {
            head = node.next.take();
            if g_count % k == 0 {

                if let Some(group) = g_head.replace(node) {

                    result_tail.insert(group);
                    while let Some(node) = result_tail {
                        result_tail = &mut node.next;
                    }
                }
            } else {
                g_head.insert(node).next = g_head.take();
            }
            g_count += 1;
        }
        if g_count % k != 0 {

            let mut tail_head:Option<Box<ListNode>> = g_head.take();
            while let Some(mut node) = tail_head {
                tail_head = node.next.take();
                g_head.insert(node).next = g_head.take();
            }
        }
        if let Some(tail) = g_head {
            let _ = result_tail.insert(tail);
        }

        result
    }
}