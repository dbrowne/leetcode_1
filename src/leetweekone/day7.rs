pub mod day_seven {

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;

        while left <= right {
            let mid: i32 = (left + right) / 2;
            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => return mid,
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid - 1,
            }
        }
        -1
    }

    pub struct Solution {
        bad_version: i32,
    }

    impl Solution {
        pub fn first_bad_version(&self, n: i32) -> i32 {
            if self.is_bad_version(1) {
                return 1;
            }
            let mut good = 1i64;
            let mut bad = n as i64; // deal with overflow
            loop {
                let mid = (good + bad) / 2;
                if self.is_bad_version(mid as i32) {
                    bad = mid;
                } else {
                    good = mid;
                }
                if good + 1 == bad {
                    return bad as i32;
                }
            }
        }

        pub fn is_bad_version(&self, n: i32) -> bool {
            self.bad_version <= n
        }
        pub fn new(bad_version: i32) -> Self {
            Self { bad_version }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetweekone::day7::day_seven::{search, Solution};

    #[test]
    fn test_1() {
        let ans = 4;
        let inp = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(ans, search(inp, 9));
    }

    #[test]
    fn test_2() {
        let ans = -1;
        let inp = vec![-1, 0, 3, 5, 9, 12, 33, 44, 99, 111, 1234, 12334, 999999];
        assert_eq!(ans, search(inp, 2));
    }

    #[test]
    fn test_3() {
        let val = Solution::new(4);
        assert_eq!(4, val.first_bad_version(5))
    }
}
