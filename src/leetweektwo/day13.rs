pub mod day_13 {
    use std::collections::HashMap;

    pub fn two_sums(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            let complement = target - nums[i];
            if num_map.contains_key(&complement) {
                return vec![num_map[&complement], i as i32];
            }
            num_map.insert(nums[i], i as i32);
        }
        vec![]
    }

    fn two_sum_fast(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(num) {
                return vec![j, i as i32];
            }
            map.insert(target - num, i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod test {
    use crate::leetweektwo::day13::day_13::two_sums;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 2], two_sums(vec![3, 2, 4], 6))
    }
}
