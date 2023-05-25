/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

/*You are given a string s and an array of strings words. All the strings of words are of the same length.

A concatenated substring in s is a substring that contains all the strings of any permutation of words concatenated.

For example, if words = ["ab","cd","ef"], then "abcdef", "abefcd", "cdabef", "cdefab", "efabcd", and "efcdab" are all concatenated strings. "acdbef" is not a concatenated substring because it is not the concatenation of any permutation of words.
Return the starting indices of all the concatenated substrings in s. You can return the answer in any order.



Example 1:

Input: s = "barfoothefoobarman", words = ["foo","bar"]
Output: [0,9]
Explanation: Since words.length == 2 and words[i].length == 3, the concatenated substring has to be of length 6.
The substring starting at 0 is "barfoo". It is the concatenation of ["bar","foo"] which is a permutation of words.
The substring starting at 9 is "foobar". It is the concatenation of ["foo","bar"] which is a permutation of words.
The output order does not matter. Returning [9,0] is fine too.
Example 2:

Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
Output: []
Explanation: Since words.length == 4 and words[i].length == 4, the concatenated substring has to be of length 16.
There is no substring of length 16 is s that is equal to the concatenation of any permutation of words.
We return an empty array.
Example 3:

Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
Output: [6,9,12]
Explanation: Since words.length == 3 and words[i].length == 3, the concatenated substring has to be of length 9.
The substring starting at 6 is "foobarthe". It is the concatenation of ["foo","bar","the"] which is a permutation of words.
The substring starting at 9 is "barthefoo". It is the concatenation of ["bar","the","foo"] which is a permutation of words.
The substring starting at 12 is "thefoobar". It is the concatenation of ["the","foo","bar"] which is a permutation of words.


Constraints:

1 <= s.length <= 104
1 <= words.length <= 5000
1 <= words[i].length <= 30
s and words[i] consist of lowercase English letters.*/


pub mod a30 {
    use std::collections::HashMap;

    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.is_empty() || s.is_empty() {
            return vec![];
        }
        let s_l = s.len();
        let w_l = words.len();  // word array
        let token_len = words[0].len();

        if s_l < token_len * w_l {
            return vec![];
        }
        let mut word_freq = words.iter().fold(HashMap::new(), |mut word_hash, word| {
            *word_hash.entry(&word[..]).or_insert(0) += 1;
            word_hash
        });

        let mut ans = vec![];
        let mut idx = 0;
        for idx in 0..token_len {
            let mut tmp_freq = word_freq.clone();
            let mut good = 0;
            // start the sliding window
            for ctr in (idx..s_l - token_len + 1).step_by(token_len) {
                let c_word = &s[ctr..ctr + token_len];
                if let Some(crumb) = tmp_freq.get(c_word) {
                    if *crumb > 0 {
                        good += 1;
                    }
                }
                *tmp_freq.entry(c_word).or_default() -= 1;
                // reduce the window size
                if ctr >= w_l * token_len {
                    let i = ctr - w_l * token_len;
                    let p_word = &s[i..i + token_len];
                    let count = tmp_freq.entry(p_word).or_default();
                    *count += 1;
                    if *count > 0 && good > 0 {
                        good -= 1;
                    }
                }
                if good == w_l {
                    ans.push((ctr + token_len - w_l * token_len) as i32);
                }
            }
        }
        ans
    }
}


#[cfg(test)]
mod test {
    use crate::general_problems::a_0030_substr_w_concat_all_words::a30::find_substring;

    #[test]
    fn t_0() {
        let s = "barfoothefoobarman".to_string();
        let words = vec!["foo".to_string(), "bar".to_string()];

        assert_eq!(vec![0, 9], find_substring(s, words));
    }

    #[test]
    fn t_1(){
        let  s ="wordgoodgoodgoodbestword".to_string();
        let words =vec!["word".to_string(),"good".to_string(),"best".to_string(),"word".to_string()];
        let  ans:Vec<i32> =vec![];
        assert_eq!(ans, find_substring(s,words));
    }
    #[test]
    fn t_2(){
        let  s = "barfoofoobarthefoobarman".to_string();
        let  words = vec!["bar".to_string(),"foo".to_string(), "the".to_string()];
        let  ans:Vec<i32> = vec![6,9,12];
        assert_eq!(ans, find_substring(s,words));
    }
}




