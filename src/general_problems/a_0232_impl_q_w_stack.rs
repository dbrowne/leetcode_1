pub mod a232 {
    struct MyQueue {
        vec0: Vec<i32>,
        vec1: Vec<i32>,
    }

    impl MyQueue {
        fn new() -> Self {
            MyQueue {
                vec0: Vec::new(),
                vec1: Vec::new(),
            }
        }

        fn push(&mut self, x: i32) {
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
}

#[cfg(test)]
mod test {
    #[test]
    fn test_01() {
        assert_eq!(0, 0);
    }
}

// mod stack {
//     pub struct Stack<T: Copy> {
//         storage: Vec<T>,
//     }
//
//     impl<T: Copy> Stack<T> {
//         pub fn new() -> Self {
//             Self { storage: vec![] }
//         }
//
//         pub fn push(&mut self, x: T) {
//             self.storage.push(x);
//         }
//
//         pub fn pop(&mut self) -> Option<T> {
//             self.storage.pop()
//         }
//
//         pub fn peek(&self) -> Option<T> {
//             self.storage.last().map(|x| x.clone())
//         }
//
//         pub fn empty(&self) -> bool {
//             self.storage.is_empty()
//         }
//     }
// }
//
// use stack::Stack;
//
// struct MyQueue {
//     real: Stack<i32>,
//     tmp: Stack<i32>,
// }
//
//
// /**
//   * `&self` means the method takes an immutable reference
//   * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl MyQueue {
//
//     fn new() -> Self {
//         Self { real: Stack::new(), tmp: Stack::new() }
//     }
//
//     fn push(&mut self, x: i32) {
//         while let Some(x) = self.real.pop() {
//             self.tmp.push(x);
//         }
//         self.real.push(x);
//         while let Some(x) = self.tmp.pop() {
//             self.real.push(x);
//         }
//     }
//
//     fn pop(&mut self) -> i32 {
//         self.real.pop().expect("All the calls to pop and peek are valid.")
//     }
//
//     fn peek(&self) -> i32 {
//         self.real.peek().expect("All the calls to pop and peek are valid.")
//     }
//
//     fn empty(&self) -> bool {
//         self.real.empty()
//     }
// }
