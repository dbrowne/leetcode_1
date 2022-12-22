pub mod day_five {
    use std::cmp;
    use std::collections::HashMap;

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let plen: usize = prices.len();
        if plen < 0 {
            return 0;
        }

        let mut profits = vec![0; plen];
        let mut max_profit = 0;
        let mut t_min = prices[0];
        for i in 1..plen {
            profits[i] = cmp::max(profits[i - 1], prices[i] - t_min);
            t_min = cmp::min(t_min, prices[i]);
            max_profit = cmp::max(profits[i], max_profit);
        }

        max_profit
    }

    pub fn longest_palindrome(s: String) -> i32 {
        let char_vec: Vec<char> = s.chars().collect();
        let mut char_map: HashMap<char, i32> = HashMap::new();

        for c in char_vec {
            *char_map.entry(c).or_insert(0) += 1;
        }
        let mut ans = 0;
        let vals: Vec<i32> = char_map.values().cloned().collect();
        for x in vals {
            ans += x / 2 * 2;
            if ans % 2 == 0 && x % 2 == 1 {
                ans += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use crate::leetweekone::day5::day_five::{longest_palindrome, max_profit};

    #[test]
    fn test_01() {
        assert_eq!(5, max_profit(vec![7, 1, 5, 3, 6, 4]))
    }

    #[test]
    fn test_02() {
        assert_eq!(7, longest_palindrome(String::from("abccccdd")));
    }

    #[test]
    fn test_03() {
        assert_eq!(1, longest_palindrome(String::from("a")));
    }
}
