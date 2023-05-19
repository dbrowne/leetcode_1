/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

pub  mod  a232{
    struct MyQueue {
        vec0: Vec<i32>,
        vec1: Vec<i32>,
    }


    /**
      * `&self` means the method takes an immutable reference
      * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

        fn new() -> Self {
            MyQueue {
                vec0: Vec::new(),
                vec1: Vec::new(),
            }
        }

        fn push(& mut self, x: i32) {
            while let Some(v) = self.vec0.pop() {
                self.vec1.push(v);
            }
            self.vec1.push(x);

            while let Some(v) = self.vec1.pop() {
                self.vec0.push(v);
            }
        }

        fn pop(&mut self) -> i32 {
            self.vec0.pop().unwrap()
        }

        fn peek(&self) -> i32 {
            *self.vec0.last().unwrap()
        }

        fn empty(&self) -> bool {
            self.vec0.is_empty()
        }
    }

    // /**
    //  * Your MyQueue object will be instantiated and called as such:
    //  * let obj = MyQueue::new();
    //  * obj.push(x);
    //  * let ret_2: i32 = obj.pop();
    //  * let ret_3: i32 = obj.peek();
    //  * let ret_4: bool = obj.empty();
    //  */


}