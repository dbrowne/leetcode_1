/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given two strings s and t, return true if they are equal when both are typed into empty text editors. '#' means a backspace character.
//
// Note that after backspacing an empty text, the text will continue empty.
//
//
//
// Example 1:
//
// Input: s = "ab#c", t = "ad#c"
// Output: true
// Explanation: Both s and t become "ac".
// Example 2:
//
// Input: s = "ab##", t = "c#d#"
// Output: true
// Explanation: Both s and t become "".
// Example 3:
//
// Input: s = "a#c", t = "b"
// Output: false
// Explanation: s becomes "c" while t becomes "b".
//
//
// Constraints:
//
// 1 <= s.length, t.length <= 200
// s and t only contain lowercase letters and '#' characters.
//
//
// Follow up: Can you solve it in O(n) time and O(1) space?

pub mod a844 {
    pub fn backspace_compare(s: String, t: String) -> bool {
        const POUND: char = '#';
        let mut s_vec: Vec<char> = vec![];
        let mut t_vec: Vec<char> = vec![];
        let s_char_iter = s.chars();
        let t_char_iter = t.chars();

        for chr in s_char_iter {
            if chr == POUND {
                if !s_vec.is_empty() {
                    let _ = s_vec.pop();
                }
            } else {
                s_vec.push(chr);
            }
        }
        for chr in t_char_iter {
            if chr == POUND {
                if !t_vec.is_empty() {
                    let _ = t_vec.pop();
                }
            } else {
                t_vec.push(chr);
            }
        }

        if t_vec.len() != s_vec.len() {
            return false;
        }

        for i in 0..t_vec.len() {
            if t_vec[i as usize] != s_vec[i as usize] {
                return false;
            }
        }
        true
    }

    pub fn backspace_compare_fast(s: String, t: String) -> bool {
        let mut bs_s = 0;
        let mut bs_t = 0;
        let mut i_s = (s.len() - 1) as i32;
        let mut i_t = (t.len() - 1) as i32;

        while i_s >= 0 || i_t >= 0 {
            let c_s = if i_s >= 0 { &s[(i_s as usize)..(i_s as usize) + 1] } else { " " };
            let c_t = if i_t >= 0 { &t[(i_t as usize)..(i_t as usize) + 1] } else { " " };
            if c_s == "#" {
                bs_s += 1;
                i_s -= 1;
                continue;
            }
            if c_t == "#" {
                bs_t += 1;
                i_t -= 1;
                continue;
            }
            if bs_s > 0 {
                bs_s -= 1;
                i_s -= 1;
                continue;
            }
            if bs_t > 0 {
                bs_t -= 1;
                i_t -= 1;
                continue;
            }
            if c_s != c_t {
                return false;
            }
            i_s -= 1;
            i_t -= 1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::leetweektwo::a_0844_backspace_string_compare::a844::backspace_compare;

    #[test]
    fn test_01() {
        assert_eq!(true, backspace_compare(String::from("ab#c"), String::from("ad#c")));
    }

    #[test]
    fn test_02() {
        assert_eq!(false, backspace_compare(String::from("a#c"), String::from("b")));
    }
}