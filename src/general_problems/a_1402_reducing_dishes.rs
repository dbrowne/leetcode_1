/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

/*A chef has collected data on the satisfaction level of his n dishes. Chef can cook any dish in 1 unit of time.

Like-time coefficient of a dish is defined as the time taken to cook that dish including previous dishes multiplied by its satisfaction level i.e. time[i] * satisfaction[i].

Return the maximum sum of like-time coefficient that the chef can obtain after dishes preparation.

Dishes can be prepared in any order and the chef can discard some dishes to get this maximum value.



Example 1:

Input: satisfaction = [-1,-8,0,5,-9]
Output: 14
Explanation: After Removing the second and last dish, the maximum total like-time coefficient will be equal to (-1*1 + 0*2 + 5*3 = 14).
Each dish is prepared in one unit of time.
Example 2:

Input: satisfaction = [4,3,2]
Output: 20
Explanation: Dishes can be prepared in any order, (2*1 + 3*2 + 4*3 = 20)
Example 3:

Input: satisfaction = [-1,-4,-5]
Output: 0
Explanation: People do not like the dishes. No dish is prepared.


Constraints:

n == satisfaction.length
1 <= n <= 500
-1000 <= satisfaction[i] <= 1000*/


pub  mod  a1402{
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        todo!()
    }
}


#[cfg(test)]
mod  test{
    use crate::general_problems::a_1402_reducing_dishes::a1402::max_satisfaction;

    #[test]
    fn t001(){
        let  inp = vec![-1,-8,0,5,-9];
        let ans = 14;

        assert_eq!(ans, max_satisfaction(inp));
    }

    #[test]
    fn t002(){
        let  inp = vec![4,3,2];
        let  ans = 20;

        assert_eq!(ans, max_satisfaction(inp));
    }

    #[test]
    fn t003(){
        let inp = vec![-1, -4, -5];
        let ans = 0;
        assert_eq!(ans, max_satisfaction(inp));
    }
}