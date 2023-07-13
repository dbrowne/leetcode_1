/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

pub mod a1235 {
    use std::collections::BTreeMap;

    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        /// take the start time, end time, and profit and put them into a vector of tuples
        /// sort the vector by end time
        /// create a BTreeMap with the key being the end time and the value being the profit
        /// iterate through the vector of tuples
        ///    get the previous profit from the BTreeMap
        ///   calculate the new profit
        ///  get the last profit from the BTreeMap
        /// if the new profit is greater than the last profit
        ///   insert the new profit into the BTreeMap
        /// return the last profit from the BTreeMap
        let mut jobs = start_time
            .into_iter()
            .zip(end_time.into_iter())
            .zip(profit.into_iter())
            .map(|((s, e), p)| (s, e, p))
            .collect::<Vec<_>>();

        jobs.sort_unstable_by_key(|&(_, end, _)| end);

        let mut dp = BTreeMap::new();

        dp.insert(0, 0);

        for (start, end, profit) in jobs {
            let &prev_profit = dp.range(..=start).last().unwrap().1;
            let new_profit = prev_profit + profit;
            let &last_profit = dp.range(..=end).last().unwrap().1;

            if new_profit > last_profit {
                dp.insert(end, new_profit);
            }
        }
        *dp.values().last().unwrap()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn t_00() {
        let start_time: Vec<i32> = vec![4, 3, 1, 2, 4, 8, 3, 3, 3, 9];
        let end_time: Vec<i32> = vec![5, 6, 3, 5, 9, 9, 8, 5, 7, 10];
        let profit = vec![9, 9, 5, 12, 10, 11, 10, 4, 14, 7];

        let ans = super::a1235::job_scheduling(start_time, end_time, profit);
        assert_eq!(ans, 37);
    }

    #[test]
    fn t_01() {
        let start_time: Vec<i32> = vec![1, 2, 3, 4, 6];
        let end_time: Vec<i32> = vec![3, 5, 10, 6, 9];
        let profit = vec![20, 20, 100, 70, 60];

        let ans = super::a1235::job_scheduling(start_time, end_time, profit);
        assert_eq!(ans, 150);
    }
}
