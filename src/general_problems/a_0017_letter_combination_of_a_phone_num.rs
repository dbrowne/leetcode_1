/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */
//
// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
//
// A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
//
//
//
//
// Example 1:
//
// Input: digits = "23"
// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
// Example 2:
//
// Input: digits = ""
// Output: []
// Example 3:
//
// Input: digits = "2"
// Output: ["a","b","c"]
//
//
// Constraints:
//
// 0 <= digits.length <= 4
// digits[i] is a digit in the range ['2', '9'].


pub mod a17 {
    use std::collections::HashMap;
    use std::mem;

    pub fn letter_combinations(digits: String) -> Vec<String> {

        // could be better. Too many clones
        let d_len = digits.len();

        if  d_len == 0 {
            return vec![];
        }

        let  mut ans: Vec<String> = Vec::new();
        let dig_vec: Vec<char> = digits.chars().collect();
        let nums: HashMap<char, Vec<char>> = HashMap::from([('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']), ('4', vec!['g', 'h', 'i']), ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']), ('7', vec!['p', 'q', 'r','s']), ('8', vec!['t', 'u','v']),
            ('9', vec!['w', 'x', 'y', 'z'])]);

        let mut char_vec: Vec<Vec<char>> = vec![];

        for i in dig_vec {
            match nums.get(&i) {
                Some(chars) => char_vec.push(chars.clone()),
                _ => (),
            }
        }

        for i in char_vec {
            if ans.is_empty() {
                ans = i.into_iter().map(|c| c.to_string()).collect();
            }else{
                mem::take(&mut ans).into_iter().for_each(|x| i.clone()
                    .into_iter()
                    .for_each(|c| ans.push(format!("{x}{c}"))));
            }
            println!("{:?}",ans);
        }

        ans
    }
}


#[cfg(test)]
mod test {
    use crate::general_problems::a_0017_letter_combination_of_a_phone_num::a17::letter_combinations;

    #[test]
    fn t_001() {
        let digits = "23".to_string();
        let ans = vec!["ad".to_string(), "ae".to_string(), "af".to_string(),
                       "bd".to_string(), "be".to_string(), "bf".to_string(),
                       "cd".to_string(), "ce".to_string(), "cf".to_string()];

        assert_eq!(ans, letter_combinations(digits));
    }
}