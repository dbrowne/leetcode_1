/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


/*
You are given an array people where people[i] is the weight of the ith person, and an
infinite number of boats where each boat can carry a maximum weight of limit. Each boat
carries at most two people at the same time, provided the sum of the weight of those
people is at most limit.

Return the minimum number of boats to carry every given person.



Example 1:

Input: people = [1,2], limit = 3
Output: 1
Explanation: 1 boat (1, 2)
Example 2:

Input: people = [3,2,2,1], limit = 3
Output: 3
Explanation: 3 boats (1, 2), (2) and (3)
Example 3:

Input: people = [3,5,3,4], limit = 5
Output: 4
Explanation: 4 boats (3), (3), (4), (5)


Constraints:

1 <= people.length <= 5 * 10^4
1 <= people[i] <= limit <= 3 * 10^4
*/

pub mod a881 {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut pc: Vec<i32> = people.clone();
        pc.sort();
        let mut i: usize = 0;
        let mut j: usize = people.len() - 1;
        let mut ans: i32 = 0;

        while i <= j {
            ans += 1;
            if (pc[i] + pc[j]) <= limit  {
                i += 1;
            }
            if j == 0 {
                break;
            }
            j -= 1;
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use crate::general_problems::a_0881_boats_to_save_people::a881::num_rescue_boats;

    #[test]
    fn t_001() {
        let inp: Vec<i32> = vec![1, 2];
        let limit: i32 = 3;
        let ans: i32 = 1;

        assert_eq!(ans, num_rescue_boats(inp, limit));
    }

    #[test]
    fn t_002() {
        let inp: Vec<i32> = vec![3, 2, 2, 1];
        let limit: i32 = 3;
        let ans: i32 = 3;
        assert_eq!(ans, num_rescue_boats(inp, limit));
    }

    #[test]
    fn t_003() {
        let inp: Vec<i32> = vec![3, 5, 3, 4];
        let limt: i32 = 5;

        let ans: i32 = 4;

        assert_eq!(ans, num_rescue_boats(inp, limt));
    }
}