/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
//
//
//
// Example 1:
//
// Input: haystack = "sadbutsad", needle = "sad"
// Output: 0
// Explanation: "sad" occurs at index 0 and 6.
// The first occurrence is at index 0, so we return 0.
// Example 2:
//
// Input: haystack = "leetcode", needle = "leeto"
// Output: -1
// Explanation: "leeto" did not occur in "leetcode", so we return -1.
//
//
// Constraints:
//
// 1 <= haystack.length, needle.length <= 104
// haystack and needle consist of only lowercase English characters.


mod  a28{

    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(i) => i as i32,
            None => -1
        }
    }
}

#[cfg(test)]
mod  test{
    use crate::leet_75_l3::a_28_find_the_index_of_first_occurrence::a28::str_str;

    #[test]
    fn t_001(){
        let  haystack = "sadbutsad".to_string() ;
        let  needle = "sad".to_string();
        let  ans = 0;

        assert_eq!(ans,str_str(haystack,needle) );
    }


    #[test]
    fn t_002(){
        let haystack = "leetcode".to_string();
        let  needle = "leeto".to_string();
        let  ans = -1;

        assert_eq!(ans,str_str(haystack,needle) );
    }
}