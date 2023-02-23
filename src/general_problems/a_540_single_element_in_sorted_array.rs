/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// You are given a sorted array consisting of only integers where every element appears exactly twice, except for one element which appears exactly once.
//
// Return the single element that appears only once.
//
// Your solution must run in O(log n) time and O(1) space.
//
//
//
// Example 1:
//
// Input: nums = [1,1,2,3,3,4,4,8,8]
// Output: 2
// Example 2:
//
// Input: nums = [3,3,7,7,10,11,11]
// Output: 10
//
//
// Constraints:
//
// 1 <= nums.length <= 105
// 0 <= nums[i] <= 105


pub  mod  a_540{

    pub  fn  single_non_duplicate(nums: Vec<i32>) -> i32{
        //based on C++ solution
        let  mut base:usize = 0;
        let  mut end:usize = nums.len()-1;
        while base < end {
            let  mut midp = base + (end-base)/2;
            if  midp %2 == 1{
                midp -=1;
            }
            if nums[midp] == nums[midp+1] {
                base = midp +2;
            } else{
                end = midp;
            }

        }
        nums[base]
    }
}


#[cfg(test)]
mod  test{
    use crate::general_problems::a_540_single_element_in_sorted_array::a_540::single_non_duplicate;

    #[test]
    fn t_001(){
        let  inp = vec![1,1,2,3,3,4,4,8,8];
        let  ans = 2;

        assert_eq!(ans,single_non_duplicate(inp) );
    }

    #[test]
    fn t_002(){
        let  inp:Vec<i32> = vec![3,3,7,7,10,11,11];
        let  ans = 10;

        assert_eq!(ans, single_non_duplicate(inp));
    }
}