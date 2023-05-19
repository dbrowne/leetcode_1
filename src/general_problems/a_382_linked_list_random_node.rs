/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

/*Given a singly linked list, return a random node's value from the linked list. Each node must have the same probability of being chosen.

Implement the Solution class:

Solution(ListNode head) Initializes the object with the head of the singly-linked list head.
int getRandom() Chooses a node randomly from the list and returns its value. All the nodes of the list should be equally likely to be chosen.


Example 1:


Input
["Solution", "getRandom", "getRandom", "getRandom", "getRandom", "getRandom"]
[[[1, 2, 3]], [], [], [], [], []]
Output
[null, 1, 3, 2, 2, 3]

Explanation
Solution solution = new Solution([1, 2, 3]);
solution.getRandom(); // return 1
solution.getRandom(); // return 3
solution.getRandom(); // return 2
solution.getRandom(); // return 2
solution.getRandom(); // return 3
// getRandom() should return either 1, 2, or 3 randomly. Each element should have equal probability of returning.


Constraints:

The number of nodes in the linked list will be in the range [1, 104].
-104 <= Node.val <= 104
At most 104 calls will be made to getRandom.


Follow up:

What if the linked list is extremely large and its length is unknown to you?
Could you solve this efficiently without using extra space?*/

pub mod a382 {

    // #[derive(PartialEq, Eq, Clone, Debug)]
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

    struct Solution {
        head: Option<Box<ListNode>>,
    }

    impl Solution {
        fn new(head: Option<Box<ListNode>>) -> Self {
            Self { head }
        }

        fn get_random(&self) -> i32 {
            use rand::*;
            let mut ans = 0;

            let mut scope: f64 = 1.0;
            let mut chosen_value = 0;
            let mut current_node = self.head.as_ref();
            ans = current_node.unwrap().val;

            let mut rng = rand::thread_rng();
            while let Some(node) = current_node {
                let y: f64 = rng.gen();

                if y < 1.0 / scope {
                    ans = node.val;
                }
                scope += 1.0;
                current_node = node.next.as_ref();
            }

            ans
        }
    }
}

// use rand::random;
//
// const MULTIPLIER: u64 = 6364136223846793005;
// const INCREMENT : u64 = 1442695040888963407;
//
// type NodeOpt = Option<Box<ListNode>>;
//
// struct Solution {
//     head: NodeOpt,
//     seed: u64,
// }
//
// impl Solution {
//
//     fn new(head: NodeOpt) -> Self {
//         Self { head, seed: random::<u64>() }
//     }
//
//     fn get_random(&mut self) -> i32 {
//         let mut seed = self.seed;
//
//         let mut quickrand = || {
//             seed = seed.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
//             seed as f64 / u64::MAX as f64
//         };
//
//         let mut node_opt = self.head.as_ref();
//         let mut rand_val = node_opt.unwrap().val;
//         let mut i = 2.0;
//
//         node_opt = node_opt.unwrap().next.as_ref();
//
//         while let Some(node) = node_opt {
//             if quickrand() < 1.0 / i {
//                 rand_val = node.val;
//             }
//             i += 1.0;
//             node_opt = node.next.as_ref();
//         }
//         self.seed = seed;
//         rand_val
//     }
// }

