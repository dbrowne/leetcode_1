/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// https://leetcode.com/problems/add-strings/

// Given two non-negative integers, num1 and num2 represented as string, return the sum of num1 and num2 as a string.
//
// You must solve the problem without using any built-in library for handling large integers (such as BigInteger). You must also not convert the inputs to integers directly.
//
//
//
// Example 1:
//
// Input: num1 = "11", num2 = "123"
// Output: "134"
// Example 2:
//
// Input: num1 = "456", num2 = "77"
// Output: "533"
// Example 3:
//
// Input: num1 = "0", num2 = "0"
// Output: "0"
//
//
// Constraints:
//
// 1 <= num1.length, num2.length <= 104
// num1 and num2 consist of only digits.
// num1 and num2 don't have any leading zeros except for the zero itself.



pub  mod a415{
    use std::cmp::max;
    pub fn add_strings(num1: String, num2: String) -> String {
        const ZERO:u8 = 0x30;
        let mut first_v: Vec<char> = num1.chars().collect();
        let mut second_v: Vec<char> = num2.chars().collect();
        let first_l = first_v.len();
        let second_l = second_v.len();
        let ans_len = max(first_l, second_l) ;
        let mut ans_v: Vec<char> = Vec::new();
        let mut carry:bool = false;
        for  idx in (0..ans_len)  {
            let  mut tmp1: u8 = 0;
            let  mut tmp2:u8 = 0;
            let  (x,y) = (first_v.pop(),second_v.pop());
            match (x,y) {
                (Some(x),Some(y)) =>{
                    tmp1 =x as u8 -ZERO;
                    tmp2 =y as u8 -ZERO;
                }
                (Some(x),None) =>{
                    tmp1 = x as u8 - ZERO;
                    tmp2 = 0;
                }
                (None,Some(y)) =>{
                    tmp2 = 0;
                    tmp1 = y as u8 - ZERO;
                }
                (None,None) =>{
                    tmp1 =0;
                    tmp2 =0;
                }
            }



            let  mut sum = tmp1 +tmp2 ;
            if  carry == true{
                sum +=1
            }
            if sum >=10 {
                carry = true;
                sum -=10;
            }else { carry = false; }

            ans_v.insert(0,(ZERO+sum) as char);


        }

        if  carry ==true{
            ans_v.insert(0,'1');
        }


        ans_v.iter().collect()
    }


    pub fn add_strings_faster(num1: String, num2: String) -> String {
        let a: Vec<char> = num1.chars().rev().collect();
        let b: Vec<char> = num2.chars().rev().collect();
        let mut c: Vec<char> = vec![];

        let mut i = 0;
        let mut j = 0;
        let mut overflow = 0;
        while i < a.len() && j < b.len() {
            let result = a[i].to_digit(10).unwrap() + b[j].to_digit(10).unwrap() + overflow;
            c.push(char::from_digit(result % 10, 10).unwrap());
            overflow = result / 10;
            i+=1;
            j+=1;
        }

        while i < a.len() {
            let result = a[i].to_digit(10).unwrap() + overflow;
            c.push(char::from_digit(result % 10, 10).unwrap());
            overflow = result / 10;
            i+=1;
        }

        while j < b.len() {
            let result = b[j].to_digit(10).unwrap() + overflow;
            c.push(char::from_digit(result % 10, 10).unwrap());
            overflow = result / 10;
            j+=1;
        }

        if overflow != 0 {
            c.push(char::from_digit(overflow, 10).unwrap());
        }

        c.iter().rev().collect()
    }

}

#[cfg(test)]
mod test {
    use crate::general_problems::a_0415_add_strings::a415::add_strings;

    #[test]
    fn test_000(){
        let  first="119".to_string();
        let  second  ="9".to_string();
        assert_eq!("128".to_string(),add_strings(first,second) );
    }

    #[test]
    fn test_001(){
        let first = "11".to_string();
        let second = "123".to_string();
        assert_eq!("134".to_string(),add_strings(first,second) );
    }

    #[test]
    fn test_002(){
        let first = "456".to_string();
        let second = "77".to_string();

        assert_eq!("533".to_string(), add_strings(first,second))

    }
}