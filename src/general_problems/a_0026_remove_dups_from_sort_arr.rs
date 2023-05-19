/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.
//
// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:
//
// Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
// Return k.
// Custom Judge:
//
// The judge will test your solution with the following code:
//
// int[] nums = [...]; // Input array
// int[] expectedNums = [...]; // The expected answer with correct length
//
// int k = removeDuplicates(nums); // Calls your implementation
//
// assert k == expectedNums.length;
// for (int i = 0; i < k; i++) {
// assert nums[i] == expectedNums[i];
// }
// If all assertions pass, then your solution will be accepted.
//
//


pub mod a26 {
    use std::collections::HashMap;
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let  mut ctr:usize = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            if map.contains_key(&nums[i]) {
                continue;
            }
            map.insert(nums[i], 1);
            nums[ctr] = nums[i];
            ctr += 1;
        }
        ctr as i32
    }

}


#[cfg(test)]
mod test {
    #[test]
    fn t001() {
        let mut nums: Vec<i32> = vec![1, 1, 2];
        let ans = 2;
        assert_eq!(ans, super::a26::remove_duplicates(&mut nums));
    }

    #[test]
    fn t002() {
        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let ans = 5;
        assert_eq!(ans, super::a26::remove_duplicates(&mut nums));
    }
}
