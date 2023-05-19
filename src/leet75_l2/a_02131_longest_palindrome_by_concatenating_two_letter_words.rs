/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/
// You are given an array of strings words. Each element of words consists of two lowercase English letters.
//
// Create the longest possible palindrome by selecting some elements from words and concatenating them in any order. Each element can be selected at most once.
//
// Return the length of the longest palindrome that you can create. If it is impossible to create any palindrome, return 0.
//
// A palindrome is a string that reads the same forward and backward.
//
//
//
// Example 1:
//
// Input: words = ["lc","cl","gg"]
// Output: 6
// Explanation: One longest palindrome is "lc" + "gg" + "cl" = "lcggcl", of length 6.
// Note that "clgglc" is another longest palindrome that can be created.
// Example 2:
//
// Input: words = ["ab","ty","yt","lc","cl","ab"]
// Output: 8
// Explanation: One longest palindrome is "ty" + "lc" + "cl" + "yt" = "tylcclyt", of length 8.
// Note that "lcyttycl" is another longest palindrome that can be created.
// Example 3:
//
// Input: words = ["cc","ll","xx"]
// Output: 2
// Explanation: One longest palindrome is "cc", of length 2.
// Note that "ll" is another longest palindrome that can be created, and so is "xx".
//
//
// Constraints:
//
// 1 <= words.length <= 105
// words[i].length == 2
// words[i] consists of lowercase English letters.

pub mod a2131 {
    use std::cmp::max;
    use std::collections::HashMap;

    pub fn longest_palindrome_failed(words: Vec<String>) -> i32 {
        let mut pairs: HashMap<String, i32> = HashMap::new();
        let mut ctr_hash: HashMap<String, i32> = HashMap::new();
        let mut ctr = 0;
        let mut center_count = 0;
        let mut has_double = false;
        for i in words {
            if is_double(&i) {
                has_double = true;
                if ctr_hash.contains_key(&i) {
                    center_count = max(center_count, *ctr_hash.get(&i).unwrap() + 1);
                    ctr_hash.entry(i.clone()).and_modify(|k| *k += 1);
                } else {
                    ctr_hash
                        .entry(i.clone())
                        .and_modify(|k| *k += 1)
                        .or_insert(1);
                }
            }
            let rev = revstr(&i);
            if pairs.contains_key(&rev) {
                if *pairs.get(&rev).unwrap() > 0 {
                    ctr += 4;
                    pairs.entry(i).and_modify(|k| *k -= 1).or_insert(0);
                    pairs.entry(rev).and_modify(|k| *k -= 1);
                }
            } else {
                pairs.entry(i).and_modify(|k| *k += 1).or_insert(1);
            }
        }

        if has_double && center_count == 0 {
            center_count = 1;
        }

        if center_count == 1 {
            center_count = 2;
        } else if center_count >= 2 && center_count % 2 == 0 {
            center_count *= 2;
        } else if center_count > 2 && center_count % 2 == 1 {
            center_count -= 1;
            center_count *= 2;
        }

        center_count + ctr
    }

    pub fn revstr(inp: &String) -> String {
        let mut chrs: Vec<char> = inp.chars().collect();

        chrs.iter().rev().collect()
    }

    pub fn is_double(inp: &String) -> bool {
        let mut chrs: Vec<char> = inp.chars().collect();
        chrs[0] == chrs[1]
    }

    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        use std::cmp::Ordering::{Equal, Greater, Less};
        let mut ctr = 0;
        let get_key = |w: &str| w.bytes().fold(0, |acc, b| acc | 1 << (b - b'a'));
        let mut dbl_h = HashMap::new();
        let mut sngl_h = HashMap::new();

        words.iter().for_each(|w| match w[..1].cmp(&w[1..]) {
            Less => sngl_h.entry(get_key(w)).or_insert((0, 0)).0 += 1,
            Greater => sngl_h.entry(get_key(w)).or_insert((0, 0)).1 += 1,
            Equal => *dbl_h.entry(w).or_insert(0) += 1,
        });

        if let Some(x) = dbl_h.values_mut().find(|x| **x % 2 == 1) {
            *x -= 1;
            ctr += 1;
        }

        // count other pairs
        dbl_h.into_values().for_each(|x| ctr += 2 * (x / 2));

        // count min equal counts
        sngl_h.values().for_each(|&(n1, n2)| ctr += 2 * n1.min(n2));

        2 * ctr
    }

    pub fn longest_palindrome_fastest(words: Vec<String>) -> i32 {
        let mut d = HashMap::new();
        for w in &words {
            *d.entry(w.as_bytes()).or_insert(0) += 1;
        }
        let mut ans = 0;
        let mut mid = 0;
        for (w, n) in &d {
            if w[0] == w[1] {
                if n % 2 == 0 {
                    ans += 2 * n;
                } else {
                    mid = 1;
                    ans += 2 * (n - 1);
                }
            } else {
                let k = vec![w[1], w[0]];
                ans += 2 * n.min(d.get(&k[..]).unwrap_or(&0));
            }
        }
        ans + 2 * mid
    }
}

#[cfg(test)]
mod test {
    use crate::leet75_l2::a_02131_longest_palindrome_by_concatenating_two_letter_words::a2131::{
        longest_palindrome, revstr,
    };

    #[test]
    fn t_000() {
        assert_eq!("ABC".to_string(), revstr(&"CBA".to_string()));
    }

    #[test]
    fn t_001() {
        assert_eq!(2, longest_palindrome(vec!["gg".to_string()]));
    }

    #[test]
    fn t_002() {
        assert_eq!(
            0,
            longest_palindrome(vec!["xq".to_string(), "ut".to_string()]),
        );
    }

    #[test]
    fn t_003() {
        assert_eq!(
            8,
            longest_palindrome(vec![
                "ab".to_string(),
                "ty".to_string(),
                "yt".to_string(),
                "lc".to_string(),
                "cl".to_string(),
                "ab".to_string()
            ])
        );
    }

    #[test]
    fn t_004() {
        assert_eq!(
            6,
            longest_palindrome(vec!["lc".to_string(), "cl".to_string(), "gg".to_string()])
        );
    }

    #[test]
    fn t_005() {
        assert_eq!(
            2,
            longest_palindrome(vec!["cc".to_string(), "ll".to_string(), "xx".to_string()])
        )
    }

    #[test]
    fn t_006() {
        assert_eq!(
            22,
            longest_palindrome(vec![
                "dd".to_string(),
                "aa".to_string(),
                "bb".to_string(),
                "dd".to_string(),
                "aa".to_string(),
                "dd".to_string(),
                "bb".to_string(),
                "dd".to_string(),
                "aa".to_string(),
                "cc".to_string(),
                "bb".to_string(),
                "cc".to_string(),
                "dd".to_string(),
                "cc".to_string()
            ])
        );
    }
}

