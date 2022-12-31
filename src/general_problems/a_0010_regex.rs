/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

//
// Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
//
// '.' Matches any single character.​​​​
// '*' Matches zero or more of the preceding element.
// The matching should cover the entire input string (not partial).
//
//
//
// Example 1:
//
// Input: s = "aa", p = "a"
// Output: false
// Explanation: "a" does not match the entire string "aa".
// Example 2:
//
// Input: s = "aa", p = "a*"
// Output: true
// Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
// Example 3:
//
// Input: s = "ab", p = ".*"
// Output: true
// Explanation: ".*" means "zero or more (*) of any character (.)".
//
//
// Constraints:
//
// 1 <= s.length <= 20
// 1 <= p.length <= 30
// s contains only lowercase English letters.
// p contains only lowercase English letters, '.', and '*'.
// It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.

pub mod a_10 {
    use std::os::unix::raw::ino_t;

    pub fn is_match(s: String, p: String) -> bool {
        let (ls, lp) = (s.len(), p.len());
        let mut dp = vec![vec![false; lp + 1]; ls + 1];
        dp[0][0] = true;
        for j in 0..lp {
            if p.bytes().nth(j).unwrap() == b'*' && dp[0][j - 1] {
                dp[0][j + 1] = true;
            }
        }
        // println!("dp = {:?}", dp);

        for i in 0..ls {
            for j in 0..lp {
                match p.bytes().nth(j).unwrap() {
                    b'.' => {
                        dp[i + 1][j + 1] = dp[i][j];
                    }
                    b'*' => {
                        if p.bytes().nth(j - 1).unwrap() != s.bytes().nth(i).unwrap() &&
                            p.bytes().nth(j - 1).unwrap() != b'.' {
                            dp[i + 1][j + 1] = dp[i + 1][j - 1];
                        } else {
                            dp[i + 1][j + 1] = dp[i + 1][j] || dp[i][j + 1] || dp[i + 1][j - 1];
                        }
                    }
                    _ => {
                        if s.bytes().nth(i).unwrap() == p.bytes().nth(j).unwrap() {
                            dp[i + 1][j + 1] = dp[i][j];
                        }
                    }
                }
            }
        }
        dp[ls][lp]
    }
}


#[cfg(test)]
mod test {
    use crate::general_problems::a_0010_regex::a_10::is_match;
    #[test]
    fn test_01() {
        assert_eq!(is_match(String::from("aa"), String::from("a")), false);
        assert_eq!(is_match(String::from("aa"), String::from("a*")), true);
        assert_eq!(is_match(String::from("ab"), String::from(".*")), true);
        assert_eq!(is_match(String::from("ab"), String::from("c*a*b")), true);
        assert_eq!(is_match(String::from("mississippi"), String::from("mis*is*p*.")), false);
        assert_eq!(is_match(String::from("mississippi"), String::from("mis*is*.p*.")), true);
    }
}