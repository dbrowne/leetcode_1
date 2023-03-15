/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
//
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
//
//
//
// Example 1:
//
// Input: strs = ["eat","tea","tan","ate","nat","bat"]
// Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
// Example 2:
//
// Input: strs = [""]
// Output: [[""]]
// Example 3:
//
// Input: strs = ["a"]
// Output: [["a"]]
//
//
// Constraints:
//
// 1 <= strs.length <= 104
// 0 <= strs[i].length <= 100
// strs[i] consists of lowercase English letters.

pub mod a49 {
    use std::collections::HashMap;
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut strs = strs.clone();
        strs.sort();
        strs.into_iter()
            .fold(HashMap::new(), |mut acc, word| {
                let val = word.chars().fold([0u8; 26], |mut acc, c| {
                    acc[c as usize - 'a' as usize] += 1;
                    acc
                });
                acc.entry(val).or_insert(vec![]).push(word);
                acc
            })
            .into_values()
            .collect()
    }

    pub fn group_anagrams_faster(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = vec![];
        let mut m: HashMap<[u8; 26], usize> = HashMap::new();
        let mut i = 0;

        for str in strs.iter() {
            let mut s: [u8; 26] = [0; 26];
            for c in str.chars() {
                let ci = c as usize - 'a' as usize;
                s[ci] += 1;
            }
            match m.get(&s) {
                Some(j) => {
                    res[*j].push(str.to_string());
                }
                None => {
                    m.insert(s, i);
                    if res.len() < i + 1 {
                        res.push(vec![]);
                    }
                    res[i].push(str.to_string());
                    i += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::leet_75_l3::a_49_group_anagrams::a49::group_anagrams;

    #[test]
    fn t_001() {
        let inp = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let ans = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
        assert_eq!(ans, group_anagrams(inp));
    }

    #[test]
    fn t_002() {
        let inp = vec!["".to_string()];
        let ans = vec![vec!["".to_string()]];
        assert_eq!(ans, group_anagrams(inp));
    }

    #[test]
    fn t_003() {
        let inp = vec!["a".to_string()];
        let ans = vec![vec!["a".to_string()]];
        assert_eq!(ans, group_anagrams(inp));
    }
}

