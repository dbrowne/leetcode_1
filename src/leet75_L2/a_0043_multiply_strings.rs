/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// https://leetcode.com/problems/multiply-strings/
//
// Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
//
// Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
//
//
//
// Example 1:
//
// Input: num1 = "2", num2 = "3"
// Output: "6"
// Example 2:
//
// Input: num1 = "123", num2 = "456"
// Output: "56088"
//
//
// Constraints:
//
// 1 <= num1.length, num2.length <= 200
// num1 and num2 consist of digits only.
// Both num1 and num2 do not contain any leading zero, except the number 0 itself.

pub mod a43 {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1: Vec<u32> = num1.bytes().map(|x| (x - b'0') as u32).collect();
        let num2: Vec<u32> = num2.bytes().map(|x| (x - b'0') as u32).collect();
        let mut res = vec![0u32; num1.len() + num2.len()];
        for j in 0..num2.len() {
            for i in 0..num1.len() {
                let sum = num1[i] * num2[j];
                res[i + j + 1] += sum % 10;
                res[i + j] += sum / 10;
            }
        }
        for i in (1..res.len()).rev() {
            res[i - 1] += res[i] / 10;
            res[i] = res[i] % 10;
        }
        let mut res: String = res.iter().fold(String::from(""), |mut acc, &x| {
            acc.push((x as u8 + '0' as u8) as char);
            acc
        });
        while !res.is_empty() && res.chars().nth(0) == Some('0') {
            res.remove(0);
        }
        println!("res = {:#?}", res);
        if res.is_empty() {
            String::from("0")
        } else {
            res
        }
    }

    pub fn multiply_faster(num1: String, num2: String) -> String {
        let num1: Vec<u8> = num1
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let num2: Vec<u8> = num2
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let mut out: Vec<u8> = Vec::new();
        out.resize(num1.len() + num2.len(), 0);

        let mut carry = 0u8;

        for (i, n1) in num1.iter().enumerate() {
            for (j, n2) in num2.iter().enumerate() {
                let target = &mut out[i + j];

                *target += n1 * n2 + carry;
                carry = *target / 10;
                *target %= 10;
            }

            let mut j = num2.len();
            while carry != 0 {
                let target = &mut out[i + j];

                *target += carry;
                carry = *target / 10;
                *target %= 10;

                j += 1;
            }
        }

        let mut out_str = String::new();
        out_str.reserve_exact(out.len());

        let out_it = out.iter().rev().skip_while(|n| **n == 0);
        out_it.for_each(|n| out_str.push(char::from_digit(*n as u32, 10).unwrap()));

        if out_str.len() == 0 {
            out_str.push('0');
        }

        out_str
    }
}

#[cfg(test)]
mod test {
    use crate::leet75_L2::a_0043_multiply_strings::a43::multiply;

    #[test]
    fn test_000() {
        assert_eq!("6", multiply("2".to_string(), "3".to_string()));
    }

    #[test]
    fn test_001() {
        assert_eq!(
            "175".to_string(),
            multiply("25".to_string(), "7".to_string())
        );
    }
}

