/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given an array of strings words and an integer k, return the k most frequent strings.
//
// Return the answer sorted by the frequency from highest to lowest. Sort the words with the same frequency by their lexicographical order.
//
//
//
// Example 1:
//
// Input: words = ["i","love","leetcode","i","love","coding"], k = 2
// Output: ["i","love"]
// Explanation: "i" and "love" are the two most frequent words.
// Note that "i" comes before "love" due to a lower alphabetical order.
// Example 2:
//
// Input: words = ["the","day","is","sunny","the","the","the","sunny","is","is"], k = 4
// Output: ["the","is","sunny","day"]
// Explanation: "the", "is", "sunny" and "day" are the four most frequent words, with the number of occurrence being 4, 3, 2 and 1 respectively.
//
//
// Constraints:
//
// 1 <= words.length <= 500
// 1 <= words[i].length <= 10
// words[i] consists of lowercase English letters.
// k is in the range [1, The number of unique words[i]]
//
//
// Follow-up: Could you solve it in O(n log(k)) time and O(n) extra space?

pub mod xyz {
    use std::collections::BinaryHeap;


    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut counter = std::collections::BTreeMap::<String, i32>::new();
        for word in words {
            *counter.entry(word).or_default() += 1;
        }
        let k = k as usize;
        let mut min_heap = BinaryHeap::new();
        for (word, freq) in counter {
            min_heap.push((freq, word));
            // if min_heap.len() > k {
            //     min_heap.pop();
            // }
        }
        // min_heap is sort by freq asc and letter desc, need to reverse it
        println!("{:?}", min_heap);
        let mut xx: Vec<(i32, String)> = min_heap.into_vec();

        println!("{:?}", xx);
        let yy: Vec<String> = xx.into_iter().map(|(freq, word)| word).collect();
        yy
    }
}

#[cfg(test)]
mod test {
    use crate::leetweektwo::a_0692_top_k_frequent_words::xyz::top_k_frequent;

    #[test]
    fn test_00() {
        let inp: Vec<String> = vec!["i".to_string(), "love".to_string(), "leetcode".to_string(),
                                    "i".to_string(), "love".to_string(), "coding".to_string()];

        assert_eq!(vec!["i".to_string(), "love".to_string()], top_k_frequent(inp, 2));
    }

    #[test]
    fn test_01() {
        let inp: Vec<String> = vec!["the".to_string(), "day".to_string(), "is".to_string(),
                                    "sunny".to_string(),
                                    "the".to_string(), "the".to_string(), "the".to_string(),
                                    "sunny".to_string(), "is".to_string(), "is".to_string()];
        let exp: Vec<String> = vec!["the".to_string(), "is".to_string(), "sunny".to_string(),
                                    "day".to_string()];
        assert_eq!(exp, top_k_frequent(inp, 4));
    }
}