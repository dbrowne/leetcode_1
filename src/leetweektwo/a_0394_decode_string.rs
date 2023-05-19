/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

/*Given an encoded string, return its decoded string.

The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.

You may assume that the input string is always valid; there are no extra white spaces, square brackets are well-formed, etc. Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there will not be input like 3a or 2[4].

The test cases are generated so that the length of the output will never exceed 105.



Example 1:

Input: s = "3[a]2[bc]"
Output: "aaabcbc"
Example 2:

Input: s = "3[a2[c]]"
Output: "accaccacc"
Example 3:

Input: s = "2[abc]3[cd]ef"
Output: "abcabccdcdcdef"


Constraints:

1 <= s.length <= 30
s consists of lowercase English letters, digits, and square brackets '[]'.
s is guaranteed to be a valid input.
All the integers in s are in the range [1, 300].*/


//Solution based on https://github.com/safrannn/leetcode_rust/blob/master/src/_0394_decode_string.rs

use std::char;

pub fn decode_string(s: String) -> String {
    let mut char_count: usize = 0;
    let string_vec: Vec<char> = s.chars().collect();

    let mut string_stack: Vec<(usize, Vec<char>)> = vec![];
    let mut res: Vec<char> = vec![];
    let n: usize = string_vec.len();

    for i in 0..n {
        let c_cur: char = string_vec[i].clone();
        if c_cur.is_ascii_digit() {
            char_count = char_count * 10 + (c_cur as usize - '0' as usize);
        } else if c_cur == '[' {
            string_stack.push((char_count, vec![]));
            char_count = 0;
        } else if c_cur == ']' {
            if let Some(h_last) = string_stack.pop() {
                for _k in 0..h_last.0 {
                    if let Some(current) = string_stack.last_mut() {
                        current.1.extend_from_slice(&h_last.1);
                    } else {
                        res.extend_from_slice(&h_last.1);
                    }
                }
            }
        } else {
            if let Some(current) = string_stack.last_mut() {
                current.1.push(c_cur);
            } else {
                res.push(c_cur);
            }
        }
    }
    res.into_iter().collect::<String>()
}


#[cfg(test)]
mod  test{
    use crate::leetweektwo::a_0394_decode_string::decode_string;

    #[test]
    fn test_00(){
        assert_eq!(String::from("aaabcbc"),decode_string(String::from("3[a]2[bc]")) );
    }

    #[test]
    fn test_01(){
        assert_eq!("accaccacc".to_string(),decode_string("3[a2[c]]".to_string()) );
    }

    #[test]
    fn test_02(){
        assert_eq!("abcabccdcdcdef".to_string(),decode_string("2[abc]3[cd]ef".to_string()) );
    }

}