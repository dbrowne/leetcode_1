/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// https://leetcode.com/problems/longest-substring-without-repeating-characters/
//
// Given a string s, find the length of the longest
// substring
// without repeating characters.
//
//
//
// Example 1:
//
// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
// Example 2:
//
// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:
//
// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//
//
// Constraints:
//
// 0 <= s.length <= 5 * 104
// s consists of English letters, digits, symbols and spaces.

pub  mod  a3{
    pub fn length_of_longest_substring(s: String) -> i32 {

        let  mut strlen:usize = 0;
        let  mut pos:[usize;128] = [0;128];
        let  mut start: usize =0;

        for (end, ch) in s.chars().enumerate(){
            start = start.max(pos[ch as usize]);
            strlen = strlen.max(end - start+1);
            pos[ch as usize] = end +1;
        }

        strlen as i32
    }
}


#[cfg(test)]
mod  test{
    use crate::leet75_L2::a_0003_longest_substr_wo_repeat::a3::length_of_longest_substring;

    #[test]
    fn t_0001(){
         assert_eq!(3,length_of_longest_substring("abcabcbb".to_string()) );
     }
}