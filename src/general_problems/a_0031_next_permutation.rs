/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */
//
// A permutation of an array of integers is an arrangement of its members into a sequence or linear order.
//
// For example, for arr = [1,2,3], the following are all the permutations of
// arr: [1,2,3], [1,3,2], [2, 1, 3], [2, 3, 1], [3,1,2], [3,2,1].
// The next permutation of an array of integers is the next lexicographically greater permutation of
// its integer. More formally, if all the permutations of the array are sorted in one container
// according to their lexicographical order, then the next permutation of that array is the
// permutation that follows it in the sorted container.
// If such arrangement is not possible, the array must be rearranged as the lowest possible
// order (i.e., sorted in ascending order).
//
// For example, the next permutation of arr = [1,2,3] is [1,3,2].
// Similarly, the next permutation of arr = [2,3,1] is [3,1,2].
// While the next permutation of arr = [3,2,1] is [1,2,3] because [3,2,1] does not have a
// lexicographical larger rearrangement.
// Given an array of integers nums, find the next permutation of nums.
//
// The replacement must be in place and use only constant extra memory.

pub  mod  a31{
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i: usize = nums.len() - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }

        if i == 0 {
            nums.reverse();
            return;
        }

        let mut j: usize = nums.len() - 1;
        while nums[j] <= nums[i - 1] {
            j -= 1;
        }

        nums.swap(i - 1, j);

        j = nums.len() - 1;
        while i < j {
            nums.swap(i, j);
            i += 1;
            j -= 1;
        }

    }

}

#[cfg(test)]
mod  tests{
    use super::*;

    #[test]
    fn test_next_permutation(){
        let mut nums: Vec<i32> = vec![1,2,3];
        a31::next_permutation(&mut nums);
        assert_eq!(nums, vec![1,3,2]);
    }
}