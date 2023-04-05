/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

/*Given a string s, partition the string into one or more substrings such that the characters in each substring are unique. That is, no letter appears in a single substring more than once.

Return the minimum number of substrings in such a partition.

Note that each character should belong to exactly one substring in a partition.



Example 1:

Input: s = "abacaba"
Output: 4
Explanation:
Two possible partitions are ("a","ba","cab","a") and ("ab","a","ca","ba").
It can be shown that 4 is the minimum number of substrings needed.
Example 2:

Input: s = "ssssss"
Output: 6
Explanation:
The only valid partition is ("s","s","s","s","s","s").
 */

pub  mod  a2405{
    pub  fn  partition_string(s: String)->i32{
        // Based on C++ solution
        const  START: u8 = b'a';
        let  mut lastSeen:Vec<i32> =vec![-1;26];
        let  mut count:i32 = 1;
        let  mut sub_start:i32 = 0;
        let  s_chars:Vec<char> = s.chars().collect();

        for idx in 0..s_chars.len()  {
            let  val:usize = (s_chars[idx] as u8 - START) as usize;
            if  lastSeen[val]  >= sub_start{
                count +=1;
                sub_start = idx as i32;
            }
            lastSeen[val] = idx as i32;

        }
        count
    }
}
#[cfg(test)]
mod  test{
    use crate::general_problems::a_2405_optimal_partition_of_string::a2405::partition_string;

    #[test]
    fn t_001(){
        let  inp:String = "abacaba".to_string();
        let  ans:i32 = 4;


        assert_eq!(ans, partition_string(inp));
    }

    #[test]
    fn t_002(){
        let  inp:String = "ssssss".to_string();
        let  ans: i32 = 6;
        assert_eq!(ans, partition_string(inp));
    }

}