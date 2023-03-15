/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/add-binary/
// Given two binary strings a and b, return their sum as a binary string.
//
//
//
// Example 1:
//
// Input: a = "11", b = "1"
// Output: "100"
// Example 2:
//
// Input: a = "1010", b = "1011"
// Output: "10101"
//
//
// Constraints:
//
// 1 <= a.length, b.length <= 104
// a and b consist only of '0' or '1' characters.
// Each string does not contain leading zeros except for the zero itself.

pub mod a67 {
    use std::cmp::max;

    pub fn add_binary(a: String, b: String) -> String {
        let a_v: Vec<char> = a.chars().collect();
        let b_v: Vec<char> = b.chars().collect();
        let max_len = max(a_v.len(), b_v.len()) + 1;
        let a_l = a.len();
        let b_l = b.len();
        let mut ans = vec!['0'; max_len];
        let mut carry = false;
        for i in (0..max_len).rev() {
            let offset = max_len - i;
            let p = if a_l < offset {
                None
            } else {
                Some(a_v[a_l - offset])
            };
            let q = if b_l < offset {
                None
            } else {
                Some(b_v[b_l - offset])
            };
            match (p, q) {
                (Some(x), Some(y)) => match (x, y) {
                    ('1', '1') => {
                        if carry {
                            ans[i] = '1'
                        }
                        carry = true
                    }
                    ('0', '1') | ('1', '0') => {
                        if !carry {
                            ans[i] = '1';
                            carry = false
                        }
                    }
                    ('0', '0') => {
                        if carry {
                            ans[i] = '1';
                            carry = false
                        }
                    }

                    _ => unreachable!(),
                },
                (Some(x), None) | (None, Some(x)) => {
                    if carry && x == '0' {
                        ans[i] = '1';
                        carry = false
                    } else if !carry {
                        ans[i] = x
                    }
                }
                _ => {
                    if carry {
                        ans[i] = '1';
                        carry = false
                    }
                }
            }
        }
        if ans[0] == '0' {
            ans.remove(0);
        }
        ans.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::general_problems::a_0067_add_binary::a67::add_binary;

    #[test]
    fn test_000() {
        assert_eq!(
            "100".to_string(),
            add_binary("11".to_string(), "1".to_string())
        );
    }

    #[test]
    fn test_001() {
        assert_eq!(
            "1110".to_string(),
            add_binary("111".to_string(), "111".to_string())
        )
    }
}
