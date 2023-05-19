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
// For example, 2 is written as II in Roman numeral, just two one's added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
//
// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
//
// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.
// Given an integer, convert it to a roman numeral.
//
//
//
// Example 1:
//
// Input: num = 3
// Output: "III"
// Explanation: 3 is represented as 3 ones.
// Example 2:
//
// Input: num = 58
// Output: "LVIII"
// Explanation: L = 50, V = 5, III = 3.
// Example 3:
//
// Input: num = 1994
// Output: "MCMXCIV"
// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
//
//
// Constraints:
//
// 1 <= num <= 3999

pub mod a12 {
    pub fn int_to_roman(num: i32) -> String {
        const SYMBOLS: &'static [(&'static str, i32);13]= &[("M", 1000), ("CM", 900), ("D", 500), ("CD", 400),
                                             ("C", 100), ("XC", 90), ("L", 50), ("XL", 40),
                                             ("X", 10), ("IX", 9), ("V", 5), ("IV", 4), ("I", 1)];
        let mut ans: String = String::new();
        let mut num = num;
        for &(numeral, val) in SYMBOLS.iter() {
            while  num >=val{
                ans.push_str(numeral);
                num -=val;

            }
        }

        ans
    }
    pub fn int_to_roman2(num: i32) -> String {
        let symbols:Vec<(&str,i32)>= vec![("M", 1000), ("CM", 900), ("D", 500), ("CD", 400),
            ("C", 100), ("XC", 90), ("L", 50), ("XL", 40),
            ("X", 10), ("IX", 9), ("V", 5), ("IV", 4), ("I", 1)];
        let mut ans: String = String::new();
        let mut num = num;
        for &(numeral, val) in symbols.iter() {
            while  num >=val{
                ans.push_str(numeral);
                num -=val;

            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use crate::general_problems::a_0012_integer_to_roman::a12::int_to_roman;

    #[test]
    fn t_001() {
        let inp = 3;
        let ans = "III".to_string();
        assert_eq!(ans, int_to_roman(inp));
    }

    #[test]
    fn t_002() {
        let inp = 58;
        let ans = "LVIII".to_string();
        assert_eq!(ans, int_to_roman(inp));
    }

    #[test]
    fn t_003() {
        let inp = 1994;
        let ans = "MCMXCIV";
    }
}