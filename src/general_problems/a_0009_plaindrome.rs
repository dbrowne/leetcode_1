/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

pub mod a_009 {
    pub fn is_palindrome(x: i32) -> bool {
        let mut stk: Vec<i32> = Vec::new();
        let mut tmp = x;

        if x < 0 {
            return false;
        }

        if x == 0 || x < 10 { return true; }
        loop {
            if tmp < 10 {
                stk.push(tmp);
                break;
            }
            let val = tmp % 10;
            stk.push(val);
            tmp = tmp / 10;
        }

        let ln = stk.len();

        let mut strt = 0;
        let mut end = ln - 1;

        loop {
            if stk[strt] != stk[end] {
                return false;
            }

            strt += 1;
            end -= 1;

            if strt > end {
                break;
            }
        }

        true
    }


    pub fn is_palindrome_fast(x: i32) -> bool {
        let x = x.to_string();
        let y: String = x.chars().rev().collect();

        x == y
    }
}

#[cfg(test)]
mod test {
    use crate::general_problems::a_0009_plaindrome::a_009::is_palindrome;

    #[test]
    fn test_01() {
        assert_eq!(true, is_palindrome(121121));
    }

    #[test]
    fn test_02() {
        assert_eq!(false, is_palindrome(12345887));
    }
}
