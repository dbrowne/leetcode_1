/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// Given a string containing just the characters '(' and ')', return the length of the longest valid (well-formed) parentheses
// substring
// .
//
//
//
// Example 1:
//
// Input: s = "(()"
// Output: 2
// Explanation: The longest valid parentheses substring is "()".
// Example 2:
//
// Input: s = ")()())"
// Output: 4
// Explanation: The longest valid parentheses substring is "()()".
// Example 3:
//
// Input: s = ""
// Output: 0
//
//
// Constraints:
//
// 0 <= s.length <= 3 * 104
// s[i] is '(', or ')'.


pub mod a32 {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.len() < 2 {
            return 0;
        }

        let mut stack: Vec<usize> = vec![];
        let s_len: usize = s.len();
        let s_chars: Vec<char> = s.chars().collect();
        let mut ans: usize = 0;
        stack.push(0);
        let mut idx: usize = 1;
        while idx <= s_len {
            if s_chars[idx - 1] == '(' {
                stack.push(idx);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(idx);
                } else {
                    ans = ans.max(idx - stack[stack.len() - 1]);
                }
            }
            idx +=1;
        }

        ans as i32
    }
}


# [cfg(test)]
mod tests {
    use crate::general_problems::a_0032_longest_valid_paren::a32::longest_valid_parentheses;

    #[test]
    fn test_1() {
        assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(longest_valid_parentheses("".to_string()), 0);
    }
}