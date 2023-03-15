/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
//
// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
// For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
//
// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
//
// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.
// Given a roman numeral, convert it to an integer.
//
//
//
// Example 1:
//
// Input: s = "III"
// Output: 3
// Explanation: III = 3.
// Example 2:
//
// Input: s = "LVIII"
// Output: 58
// Explanation: L = 50, V= 5, III = 3.
// Example 3:
//
// Input: s = "MCMXCIV"
// Output: 1994
// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
//
//
// Constraints:
//
// 1 <= s.length <= 15
// s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
// It is guaranteed that s is a valid roman numeral in the range [1, 3999].

// 'V' => integer = if previous_char == 'I' { integer + 3 } else { integer + 5 },
// 'X' => integer = if previous_char == 'I' { integer + 8 } else { integer + 10 },
// 'L' => integer = if previous_char == 'X' { integer + 30 } else { integer + 50 },
// 'C' => integer = if previous_char == 'X' { integer + 80 } else { integer + 100 },
// 'D' => integer = if previous_char == 'C' { integer + 300 } else { integer + 500 },
// 'M' => integer = if previous_char == 'C' { integer + 800 } else { integer + 1000 },
// _ => return 0

pub mod a_13 {

    pub fn roman_to_int(s: String) -> i32 {
        let mut ans: i32 = 0;
        // a bit ugly but it works
        let mut s_vec: Vec<char> = s.chars().rev().collect();
        let mut first: Option<char> = s_vec.pop();

        loop {
            let mut second: Option<char> = s_vec.pop();
            match (first, second) {
                (Some('I'), Some('V') | Some('X')) => ans -= 1,
                (Some('X'), Some('L') | Some('C')) => ans -= 10,
                (Some('C'), Some('D') | Some('M')) => ans -= 100,
                (Some('I'), _) => ans += 1,
                (Some('V'), _) => ans += 5,
                (Some('X'), _) => ans += 10,
                (Some('L'), _) => ans += 50,
                (Some('C'), _) => ans += 100,
                (Some('D'), _) => ans += 500,
                (Some('M'), _) => ans += 1000,
                (None, None) => break,
                _ => unreachable!(),
            }

            first = second;
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use crate::general_problems::a_0013_integer_to_roman::a_13::roman_to_int;

    #[test]
    fn t_00() {
        let inp: String = "III".to_string();
        let ans = 3;
        assert_eq!(ans, roman_to_int(inp));
    }

    #[test]
    fn t_01() {
        let inp: String = "LVIII".to_string();
        let ans = 58;
        assert_eq!(ans, roman_to_int(inp));
    }

    #[test]
    fn t_02() {
        let inp: String = "MCMXCIV".to_string();
        let ans = 1994;
        assert_eq!(ans, roman_to_int(inp));
    }
}

