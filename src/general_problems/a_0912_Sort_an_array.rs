/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given an array of integers nums, sort the array in ascending order and return it.
//
// You must solve the problem without using any built-in functions in O(nlog(n)) time complexity and with the smallest space complexity possible.
//
//
//
// Example 1:
//
// Input: nums = [5,2,3,1]
// Output: [1,2,3,5]
// Explanation: After sorting the array, the positions of some numbers are not changed (for example, 2 and 3), while the positions of other numbers are changed (for example, 1 and 5).
// Example 2:
//
// Input: nums = [5,1,1,2,0,0]
// Output: [0,0,1,1,2,5]
// Explanation: Note that the values of nums are not necessairly unique.
//
//
// Constraints:
//
// 1 <= nums.length <= 5 * 104
// -5 * 104 <= nums[i] <= 5 * 104

pub  mod  a912{
    pub fn sort_array2(nums: Vec<i32>) -> Vec<i32> {
        let  mut ans:Vec<i32> = nums.clone();

        qsort(&mut ans);

        ans
    }

    fn qsort(inp: &mut [i32]) {
        if inp.len() <= 1 {
            return;
        }
        let pivot_index:usize = partition(inp);
        qsort(&mut inp[..pivot_index]);
        qsort(&mut inp[pivot_index + 1..]);
    }

    fn partition(arr: &mut [i32]) -> usize {
        let pvt_idx:usize = arr.len() - 1;
        let pvt = arr[pvt_idx];

        let mut i = 0;
        for j in 0..pvt_idx {
            if arr[j] < pvt {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, pvt_idx);
        i
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        const MAXVEC:usize=100001;
        const MAXVAL:i32=50000;
        let mut cnt:[i32;MAXVEC] = [0; MAXVEC];
        let num_len:usize = nums.len();

        for n in nums {
            cnt[(n+ MAXVAL) as usize ] += 1;
        }
        let mut ans = Vec::with_capacity(num_len);
        for i in 0..cnt.len() {
            for _ in 0..cnt[i] {
                ans.push(i as i32 - MAXVAL);
            }
        }
        return ans;
    }


}

#[cfg(test)]
mod  test{
    use crate::general_problems::a_0912_Sort_an_array::a912::sort_array;

    #[test]
    fn t_001(){
        let  inp:Vec<i32> = vec![5,2,3,1];
        let  ans:Vec<i32> = vec![1,2,3,5];
        assert_eq!(ans, sort_array(inp));
    }

    #[test]
    fn t_0002(){
        let  inp:Vec<i32> = vec![5,1,1,2,0,0];
        let  ans:Vec<i32> = vec![0,0,1,1,2,5];
        assert_eq!(ans,sort_array(inp) );
    }
}