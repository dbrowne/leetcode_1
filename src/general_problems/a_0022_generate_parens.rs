/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
//
//
//
// Example 1:
//
// Input: n = 3
// Output: ["((()))","(()())","(())()","()(())","()()()"]
// Example 2:
//
// Input: n = 1
// Output: ["()"]
//
//
// Constraints:
//
// 1 <= n <= 8

pub mod a22 {

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        //backtracking
        let mut ans: Vec<String> = vec![];
        backtrack("", 0, 0, n, &mut ans);

        ans
    }

    pub fn backtrack(res_str: &str, left: i32, right: i32, max: i32, res: &mut Vec<String>) {
        if res_str.len() as i32 == max * 2 {
            res.push(res_str.to_string());
            return;
        }
        if left < max {
            backtrack(&format!("{}(", res_str), left + 1, right, max, res);
        }
        if right < left {
            backtrack(&format!("{})", res_str), left, right + 1, max, res);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::general_problems::a_0022_generate_parens::a22::generate_parenthesis;

    fn t_001() {
        let inp: i32 = 3;
        let ans: Vec<String> = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        assert_eq!(ans, generate_parenthesis(inp));
    }
}

