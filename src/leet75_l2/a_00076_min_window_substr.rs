/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/minimum-window-substring
// Given two strings s and t of lengths m and n respectively, return the minimum window
// substring
// of s such that every character in t (including duplicates) is included in the window. If there is no such substring, return the empty string "".
//
// The testcases will be generated such that the answer is unique.
//
//
//
// Example 1:
//
// Input: s = "ADOBECODEBANC", t = "ABC"
// Output: "BANC"
// Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.
// Example 2:
//
// Input: s = "a", t = "a"
// Output: "a"
// Explanation: The entire string s is the minimum window.
// Example 3:
//
// Input: s = "a", t = "aa"
// Output: ""
// Explanation: Both 'a's from t must be included in the window.
// Since the largest window of s only has one 'a', return empty string.
//
//
// Constraints:
//
// m == s.length
// n == t.length
// 1 <= m, n <= 105
// s and t consist of uppercase and lowercase English letters.
//
//
// Follow up: Could you find an algorithm that runs in O(m + n) time?

pub  mod  a76{
    use std::collections::HashMap;
    pub fn min_window(s: String, t: String) -> String {
        let mut m = HashMap::new();
        t.bytes().for_each(|c| *m.entry(c).or_insert(0) += 1);

        let mut rez: Option<&[u8]> = None;
        let mut beg = 0;
        let s = s.into_bytes();

        for (end, &end_char) in s.iter().enumerate() {
            m.entry(end_char).and_modify(|n| *n -= 1);
            if rez.is_some() || m.values().all(|n| *n <= 0) {
                while m.get(&s[beg]).copied().unwrap_or(-42) < 0 {
                    m.entry(s[beg]).and_modify(|n| *n += 1);
                    beg += 1;
                }
                if rez.is_none() || rez.unwrap().len() > end - beg {
                    rez = Some(&s[beg..=end]);
                }
            }
        }
        String::from_utf8(rez.unwrap_or_default().to_vec()).unwrap()
    }



    pub fn min_window_faster(s: String, t: String) -> String {
        let s = s.chars().collect::<Vec<char>>();

        let char_index = |c: char| {
            if ('a'..='z').contains(&c) {
                return c as usize - b'a' as usize
            } else if ('A'..='Z').contains(&c) {
                return c as usize - b'A' as usize + 26
            } else {
                panic!("shouldn't be able to get here");
            }
        };

        // Do some preprocessing first on t. Convert it into an array [usize, 52] of occurrence
        // counts for lower / upper case letters.
        let mut tt = [0; 52];
        t.chars().for_each(|c| tt[char_index(c)] += 1);

        // tracks the number of occurrences of each character in our current window
        let mut st = [0; 52];

        let mut window = 0..0;
        let mut minimum_window: Option<std::ops::Range<usize>> = None;

        loop {
            // extend the window to the right until each element of st is larger than the
            // corresponding element of tt (meaning that this is a valid window).
            loop {
                // If we hit the end of the string at this point, just return.
                if window.end == s.len() {
                    if let Some(w) = minimum_window {
                        return s[w].iter().collect();
                    } else {
                        return "".to_string();
                    }
                }

                window.end += 1;
                let ci = char_index(s[window.end - 1]);
                st[ci] += 1;

                // Check if this is now a valid window.
                if st[ci] == tt[ci] && (0..52).all(|i| st[i] >= tt[i]) {
                    break;
                }
            }

            // Start eliminating characters from the left, until the window is no longer valid. At
            // that point, we'll have the minimum length (valid) window that finishes just before
            // window.end.
            loop {
                if window.start == window.end {
                    break;
                }

                // detect when eliminating the next char would invalidate the window
                let ci = char_index(s[window.start]);
                if st[ci] == tt[ci] {
                    // This is the locally-minimum valid window; record it.
                    if let Some(ref mut w) = minimum_window {
                        if window.len() < w.len() {
                            *w = window.clone();
                        }
                    } else {
                        minimum_window = Some(window.clone());
                    }

                    st[ci] -= 1;
                    window.start += 1;
                    break;
                }

                st[ci] -= 1;
                window.start += 1;
            }
        }
    }
}

#[cfg(test)]
mod  test{
    use crate::leet75_l2::a_00076_min_window_substr::a76::min_window;

    #[test]
    fn t_0(){
        let  inp="ADOBECODEBANC".to_string();
        let  t = "ABC".to_string();
        let  ans = "BANC".to_string();
        assert_eq!(ans,min_window(inp,t));
    }
}