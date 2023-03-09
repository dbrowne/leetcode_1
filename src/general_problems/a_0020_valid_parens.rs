/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//
// An input string is valid if:
//
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.
//
//
// Example 1:
//
// Input: s = "()"
// Output: true
// Example 2:
//
// Input: s = "()[]{}"
// Output: true
// Example 3:
//
// Input: s = "(]"
// Output: false
//
//
// Constraints:
//
// 1 <= s.length <= 104
// s consists of parentheses only '()[]{}'.

pub  mod  a20{

    pub  fn  is_valid(s:String) -> bool{
        let  mut pair:Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' => pair.push(')'),
                '{' => pair.push('}'),
                '[' => pair.push(']'),
                left => if Some (left) != pair.pop() {
                    return false
                }
            }
        }
        pair.is_empty()
    }
}

#[cfg(test)]
mod  test{
    use crate::general_problems::a_0020_valid_parens::a20::is_valid;

    #[test]
    fn t_001(){
        let  s:String = "()".to_string();
        let  ans = true;

        assert_eq!(ans, is_valid(s));
    }

    #[test]
    fn t_002(){
        let  s:String = "()[]{}".to_string();
        let  ans = true;

        assert_eq!(ans, is_valid(s));

    }

    #[test]
    fn t_003(){
        let  s:String="(]".to_string();
        let ans = false;

        assert_eq!(ans, is_valid(s));
    }
}