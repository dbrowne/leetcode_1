/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
//
// Implement the MinStack class:
//
// MinStack() initializes the stack object.
// void push(int val) pushes the element val onto the stack.
// void pop() removes the element on the top of the stack.
// int top() gets the top element of the stack.
// int getMin() retrieves the minimum element in the stack.
// You must implement a solution with O(1) time complexity for each function.
//
//
//
// Example 1:
//
// Input
// ["MinStack","push","push","push","getMin","pop","top","getMin"]
// [[],[-2],[0],[-3],[],[],[],[]]
//
// Output
// [null,null,null,null,-3,null,0,-2]
//
// Explanation
// MinStack minStack = new MinStack();
// minStack.push(-2);
// minStack.push(0);
// minStack.push(-3);
// minStack.getMin(); // return -3
// minStack.pop();
// minStack.top();    // return 0
// minStack.getMin(); // return -2
//
//
// Constraints:
//
// -231 <= val <= 231 - 1
// Methods pop, top and getMin operations will always be called on non-empty stacks.
// At most 3 * 104 calls will be made to push, pop, top, and getMin.

pub  mod  a155{
    struct MinStack {
        s: Vec<(i32, i32)>
    }
    /**
      * `&self` means the method takes an immutable reference
      * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

        fn new() -> Self {
            MinStack {
                s: Vec::with_capacity(30000)
            }
        }

        fn push(&mut self, val: i32) {
            if self.s.is_empty() {
                self.s.push((val, val));
            } else {
                let min = std::cmp::min(val, self.s[self.s.len() - 1].1);
                self.s.push((val, min));
            }
        }

        fn pop(&mut self) {
            self.s.pop();
        }

        fn top(&self) -> i32 {
            self.s.last().unwrap().0
        }

        fn get_min(&self) -> i32 {
            self.s.last().unwrap().1
        }
    }

    /**
     * Your MinStack object will be instantiated and called as such:
     * let obj = MinStack::new();
     * obj.push(val);
     * obj.pop();
     * let ret_3: i32 = obj.top();
     * let ret_4: i32 = obj.get_min();
     */



    use std::cmp::min;

    #[derive(Debug)]
    struct MinStack2 {
        q: Vec<i32>,
        mins: Vec<i32>,
    }

    /**
     * `&self` means the method takes an immutable reference
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl MinStack2 {
        fn new() -> Self {
            Self {
                q: Default::default(),
                mins: Default::default(),
            }
        }

        fn push(&mut self, val: i32) {
            self.q.push(val);
            match self.mins.last() {
                None => self.mins.push(val),
                Some(old) => {
                    let new_min = min(*old, val);
                    self.mins.push(new_min);
                }
            }
        }

        fn pop(&mut self) {
            self.q.pop().unwrap();
            self.mins.pop().unwrap();
        }

        fn top(&self) -> i32 {
            *self.q.last().unwrap()
        }

        fn get_min(&self) -> i32 {
            *self.mins.last().unwrap()
        }
    }

}