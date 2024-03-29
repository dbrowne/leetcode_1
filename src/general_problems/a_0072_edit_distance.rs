/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.
//
// You have the following three operations permitted on a word:
//
// Insert a character
// Delete a character
// Replace a character
//
//
// Example 1:
//
// Input: word1 = "horse", word2 = "ros"
// Output: 3
// Explanation:
// horse -> rorse (replace 'h' with 'r')
// rorse -> rose (remove 'r')
// rose -> ros (remove 'e')
// Example 2:
//
// Input: word1 = "intention", word2 = "execution"
// Output: 5
// Explanation:
// intention -> inention (remove 't')
// inention -> enention (replace 'i' with 'e')
// enention -> exention (replace 'n' with 'x')
// exention -> exection (replace 'n' with 'c')
// exection -> execution (insert 'u')
//
//
// Constraints:
//
// 0 <= word1.length, word2.length <= 500
// word1 and word2 consist of lowercase English letters.

mod  a72{
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let w_l1 = word1.len();
        let w_l2 = word2.len();
        if w_l1 == 0 || w_l2 == 0 {
            return (w_l1 | w_l2) as i32;
        };
        let mut ans:Vec<Vec<i32>> = vec![vec![0i32; (w_l2 + 1) as usize]; (w_l1 + 1) as usize];
        for i in 0..=w_l1 {
            for j in 0..=w_l2 {
                ans[i][j] = if i == 0 || j == 0 {
                    (j | i) as i32
                } else if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] {
                    ans[i - 1][j - 1]
                } else {
                    1 + ans[i - 1][j - 1].min(ans[i - 1][j].min(ans[i][j - 1]))
                }
            }
        }
        return ans[w_l1][w_l2];
    }

}