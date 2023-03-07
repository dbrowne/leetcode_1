/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given an array arr of positive integers sorted in a strictly increasing order, and an integer k.
//
// Return the kth positive integer that is missing from this array.
//
//
//
// Example 1:
//
// Input: arr = [2,3,4,7,11], k = 5
// Output: 9
// Explanation: The missing positive integers are [1,5,6,8,9,10,12,13,...]. The 5th missing positive integer is 9.
// Example 2:
//
// Input: arr = [1,2,3,4], k = 2
// Output: 6
// Explanation: The missing positive integers are [5,6,7,...]. The 2nd missing positive integer is 6.
//
//
// Constraints:
//
// 1 <= arr.length <= 1000
// 1 <= arr[i] <= 1000
// 1 <= k <= 1000
// arr[i] < arr[j] for 1 <= i < j <= arr.length
//
//
// Follow up:
//
// Could you solve this problem in less than O(n) complexity?

pub  mod  a1539{
    pub  fn  find_kth_positive_(arr: Vec<i32>, k:i32) ->i32{
        // A binary search works for finding the kth missing element in a sorted array of increasing
        // unique integers because it takes advantage of the fact that the array is already sorted.
        //
        // First, we can determine the number of missing elements between the first element of the
        // array and the current mid-point of the array using the following formula:
        //
        //     missing_count = array[mid] - array[0] - mid
        //
        // Here, mid is the index of the current mid-point of the array. The missing_count gives us
        // the number of missing elements between the first element of the array and the current m
        // id-point of the array.
        //
        // We can then compare this missing_count with the desired value of k to determine whether
        // the kth missing element lies in the left or right half of the array.
        //
        //                                                                                                                                                                                                                                                                                                                                                                         If missing_count is less than k, then the kth missing element must lie in the right half of the array. We can then update our search range to only consider the right half of the array, by setting left = mid + 1.
        //
        // If missing_count is greater than or equal to k, then the kth missing element must lie in
        // the left half of the array. We can then update our search range to only consider the left
        // half of the array, by setting right = mid.
        //
        //  We repeat this process until we find the kth missing element or until our search range
        // becomes invalid.
        //
        // This works because the number of missing elements between the first element of the array
        // and any element in the array is a monotonically increasing function of the index of that
        // element. As a result, we can use binary search to efficiently find the element that
        // corresponds to the kth missing element.                                                                                                                                                                                                                               We repeat this process until we find the kth missing element or until our search range becomes invalid.


        let mut left:usize =0;
        let  mut right:usize=arr.len();

        while left < right {
            let mid:usize = left + (right - left) / 2;
            if arr[mid] - mid as i32 - 1 >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32 + k
    }

}

#[cfg(test)]
mod  test{
    use crate::general_problems::a_1539_kth_missing_positive_num::a1539::find_kth_positive_;

    #[test]
    fn t_001(){
        let  inp:Vec<i32> = vec![2,3,4,7,11];
        let  k = 4;
        let  ans =8;

        assert_eq!(ans, find_kth_positive_(inp,k));
    }


    #[test]
    fn t_002(){
        let  inp:Vec<i32> = vec![1,2,3,4];
        let  k = 3;
        let  ans =7;
        assert_eq!(ans, find_kth_positive_(inp,k));
    }
}