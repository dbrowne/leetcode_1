/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// Given a string s which represents an expression, evaluate this expression and return its value.
//
// The integer division should truncate toward zero.
//
// You may assume that the given expression is always valid. All intermediate results will be in the range of [-231, 231 - 1].
//
// Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().
//
//
//
// Example 1:
//
// Input: s = "3+2*2"
// Output: 7
// Example 2:
//
// Input: s = " 3/2 "
// Output: 1
// Example 3:
//
// Input: s = " 3+5 / 2 "
// Output: 5
//
//
// Constraints:
//
// 1 <= s.length <= 3 * 105
// s consists of integers and operators ('+', '-', '*', '/') separated by some number of spaces.
// s represents a valid expression.
// All the integers in the expression are non-negative integers in the range [0, 231 - 1].
// The answer is guaranteed to fit in a 32-bit integer.


mod  a227{
    pub fn calculate(s: String) -> i32 {
        enum Operator {
            Add,
            Sub,
            Mul,
            Div,
            Ext,
        }

        let mut stack: Vec<i32> = Vec::new();
        let mut n = 0;
        let mut op = Operator::Add;
        for c in (s + ".").chars() {
            if c == ' ' {
                continue;
            }
            if ('0'..='9').contains(&c) {
                n = n * 10 + (c as u8 - b'0') as i32;
            } else {
                match op {
                    Operator::Add => stack.push(n),
                    Operator::Sub => stack.push(-n),
                    Operator::Mul => *stack.last_mut().unwrap() *= n,
                    Operator::Div => *stack.last_mut().unwrap() /= n,
                    Operator::Ext => {}
                };
                op = match c {
                    '+' => Operator::Add,
                    '-' => Operator::Sub,
                    '*' => Operator::Mul,
                    '/' => Operator::Div,
                    _ => Operator::Ext,
                };
                n = 0;
            }
        }
        stack.iter().sum()
    }


    pub fn calculate2(s: String) -> i32 {
        let chars: Vec<char> = s.chars().into_iter().collect();
        let len = chars.len();
        let mut stack = vec![];
        let mut current_number = 0;
        let mut operation = '+';
        for i in 0..len {
            let current_char = chars[i];
            if current_char.is_ascii_digit() {
                current_number = current_number * 10 + (current_char.to_digit(10).unwrap() as i32);
            }
            if !current_char.is_ascii_digit()
                && !current_char.is_ascii_whitespace()
                || i == len - 1 {
                if operation == '-' {
                    stack.push(-1 * current_number);
                } else if operation == '+' {
                    stack.push(current_number);
                } else if operation == '*' {
                    let num = stack.pop().unwrap();
                    stack.push(num * current_number);
                } else if operation == '/' {
                    let num = stack.pop().unwrap();
                    stack.push((num  as f64 / current_number as f64).trunc() as i32);
                }
                operation = current_char;
                current_number = 0;
            }
        }
        stack.iter().sum()
    }
}

#[cfg(test)]
mod  test{
    use crate::leet75_L2::a_00227_basic_calculator_II::a227::calculate;

    #[test]
    fn t_001(){
        assert_eq!(7,calculate("3+2*2".to_string()));
    }
}