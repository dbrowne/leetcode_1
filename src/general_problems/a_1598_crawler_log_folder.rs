/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// The Leetcode file system keeps a log each time some user performs a change folder operation.
//
// The operations are described below:

// "../" : Move to the parent folder of the current folder. (If you are already in the main folder, remain in the same folder).
// "./" : Remain in the same folder.
// "x/" : Move to the child folder named x (This folder is guaranteed to always exist).
// You are given a list of strings logs where logs[i] is the operation performed by the user at the ith step.
//
// The file system starts in the main folder, then the operations in logs are performed.
//
// Return the minimum number of operations needed to go back to the main folder after the change folder operations.
//
//
//
// Example 1:
//
//
//
// Input: logs = ["d1/","d2/","../","d21/","./"]
// Output: 2
// Explanation: Use this change folder operation "../" 2 times and go back to the main folder.
// Example 2:
//
//
//
// Input: logs = ["d1/","d2/","./","d3/","../","d31/"]
// Output: 3
// Example 3:
//
// Input: logs = ["d1/","../","../","../"]
// Output: 0
//
//
// Constraints:
//
// 1 <= logs.length <= 103
// 2 <= logs[i].length <= 10
// logs[i] contains lowercase English letters, digits, '.', and '/'.
// logs[i] follows the format described in the statement.
// Folder names consist of lowercase English letters and digits.


pub fn min_operations(logs: Vec<String>) -> i32 {
    const CURDIR: &str = "./";
    const BACK: &str = "../";

    let mut depth: Vec<i32> = vec![];

    for tok in logs.iter() {
        match tok.as_str() {
            BACK => {
                if !depth.is_empty() {
                    depth.pop();
                }
            }
            CURDIR => (),
            _ => depth.push(1),
        }
    }

    depth.len() as i32
}

pub  fn  min_operation_no_stack(logs: Vec<String>) ->i32{

        let  mut depth = 0;
        const CURDIR: &str = "./";
        const BACK: &str = "../";

    for tok in logs   {
        match tok.as_str() {
            BACK => depth = 0.max(depth -1),
            CURDIR => (),
            _ => depth +=1,

        }

    }
    depth


}

#[cfg(test)]
mod test {
    use crate::general_problems::a_1598_crawler_log_folder::{min_operation_no_stack, min_operations};
    #[test]
    fn test_01() {
        let  inp:Vec<String> =  vec![String::from("d1/"),String::from("d2/"),String::from("../"),
            String::from("d21/"),String::from("./")];
        assert_eq!(2, min_operations(inp));
    }

    #[test]
    fn test_02(){
        let  inp:Vec<String> = vec![String::from("d1/"),String::from("d2/"),
                                    String::from("./"),String::from("d3/"),
                                    String::from("../"),String::from("d31/")];
        assert_eq!(3,min_operations(inp) );
    }

    #[test]
    fn test_03(){
        let inp: Vec<String> = vec![String::from("d1/"),String::from("../"),
                                    String::from("../"),String::from("../")];
        assert_eq!(0,min_operations(inp) );
    }

    #[test]
    fn test_04() {
        let  inp:Vec<String> =  vec![String::from("d1/"),String::from("d2/"),String::from("../"),
                                     String::from("d21/"),String::from("./")];
        assert_eq!(2, min_operation_no_stack(inp));
    }

    #[test]
    fn test_05(){
        let  inp:Vec<String> = vec![String::from("d1/"),String::from("d2/"),
                                    String::from("./"),String::from("d3/"),
                                    String::from("../"),String::from("d31/")];
        assert_eq!(3,min_operation_no_stack(inp) );
    }

    #[test]
    fn test_06(){
        let inp: Vec<String> = vec![String::from("d1/"),String::from("../"),
                                    String::from("../"),String::from("../")];
        assert_eq!(0,min_operation_no_stack(inp) );
    }
}