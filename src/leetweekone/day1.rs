pub mod day_one {
    use std::collections::HashMap;

    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut outp: Vec<i32> = Vec::new();
        let mut ctr = 0;
        for x in nums.iter() {
            if ctr == 0 {
                outp.push(*x);
                ctr += 1;
            } else {
                outp.push(*x + outp[ctr - 1]);
                ctr += 1;
            }
        }

        outp
    }

    pub fn running_sum_fast(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut sum = Vec::with_capacity(len);
        let mut i = 0;
        sum.push(nums[i]);
        i += 1;

        while i < len {
            sum.push(sum[i - 1] + nums[i]);
            i += 1;
        }
        sum
    }

    pub fn count_chars(inp: String) -> HashMap<char, usize> {
        let result = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

        result
            .chars()
            .map(|c| (c, inp.matches(c).count()))
            .collect::<HashMap<_, _>>()
    }
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut lsum = 0;
        let tot_sum: i32 = nums.iter().sum();
        for x in 0..nums.len() {
            if lsum == (tot_sum - lsum - nums[x]) {
                return x as i32;
            }
            lsum += nums[x];
        }
        -1
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let merged_len = nums1.len() + nums2.len();
        let mut merged_vec: Vec<i32> = Vec::new();
        let mut f_idx = 0; // index for nums 1  first index
        let mut s_idx = 0; // index for nums 2  second index
        let n1l = nums1.len();
        let n2l = nums2.len();
        while f_idx < n1l || s_idx < n2l {
            if f_idx == n1l {
                merged_vec.push(nums2[s_idx]);
                s_idx += 1;
            } else if s_idx == n2l {
                merged_vec.push(nums1[f_idx]);
                f_idx += 1;
            } else {
                if nums1[f_idx] <= nums2[s_idx] {
                    merged_vec.push(nums1[f_idx]);
                    f_idx += 1;
                } else {
                    merged_vec.push(nums2[s_idx]);
                    s_idx += 1;
                }
            }
        }

        let midp = merged_len / 2;

        if merged_len % 2 == 0 {
            return (merged_vec[midp - 1] + merged_vec[midp]) as f64 / 2.0;
        }

        merged_vec[midp] as f64
    }
}

#[cfg(test)]
mod tests {
    use crate::leetweekone::day1::day_one::{
        count_chars, find_median_sorted_arrays, pivot_index, running_sum,
    };
    use std::collections::HashMap;
    #[test]
    fn test_0() {
        let mapp = count_chars(String::from("Hellllo"));
        let xx = HashMap::from([('H', 1), ('e', 1), ('l', 4), ('o', 1)]);

        for k in xx.keys() {
            assert_eq!(xx[k], mapp[k]);
        }
    }

    #[test]
    fn test_1() {
        let frst: Vec<i32> = vec![1, 3, 6, 10];
        let xx = running_sum(vec![1, 2, 3, 4]);

        for x in 0..xx.len() {
            assert_eq!(frst[x], xx[x]);
        }
    }

    #[test]
    fn test_2() {
        let inp: Vec<i32> = vec![1, 3, 7, 6, 5, 6];
        let tstval = pivot_index(inp);
        assert_eq!(3, tstval)
    }

    #[test]
    fn test_3() {
        let xx = find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(xx, 2.0)
    }
}
