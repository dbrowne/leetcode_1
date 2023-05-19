/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.
//
//
//
// Example 1:
//
// Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
// Output: [[1,6],[8,10],[15,18]]
// Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
// Example 2:
//
// Input: intervals = [[1,4],[4,5]]
// Output: [[1,5]]
// Explanation: Intervals [1,4] and [4,5] are considered overlapping.
//
//
// Constraints:
//
// 1 <= intervals.length <= 104
// intervals[i].length == 2
// 0 <= starti <= endi <= 104

mod  a56{
    trait Interval<T> {
        fn is_overlap(&self, other: &T) -> bool;
        fn is_in(&self, x: i32) -> bool;
        fn merge(self, other: T) -> Self;
    }

    impl Interval<(i32, i32)> for (i32, i32) {
        fn is_overlap(&self, other: &(i32, i32)) -> bool {
            let (start, end) = *other;
            self.is_in(start) || self.is_in(end)
        }

        fn is_in(&self, x: i32) -> bool {
            self.0 <= x && x <= self.1
        }

        fn merge(self, other: Self) -> Self {
            (self.0.min(other.0), self.1.max(other.1))
        }
    }
    pub fn merge2(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals
            .into_iter()
            .map(|vec| (vec[0], vec[1]))
            .collect::<Vec<(i32, i32)>>();
        intervals.sort();

        let (mut results, last_interval) =
            intervals
                .into_iter()
                .fold(
                    (vec![], None),
                    |(mut results, prev_interval), interval| match prev_interval {
                        None => (results, Some(interval)),
                        Some(prev) if prev.is_overlap(&interval) => {
                            (results, Some(interval.merge(prev)))
                        }
                        Some(prev) => {
                            results.push(prev);
                            (results, Some(interval))
                        }
                    },
                );
        if let Some(interval) = last_interval {
            results.push(interval);
        }
        results
            .into_iter()
            .map(|(start, end)| vec![start, end])
            .collect()
    }

    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let  mut intervals = intervals.clone();
        if intervals.len() <= 1 {
            return intervals;
        }
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut result = Vec::new();
        let (mut left, mut right) = (intervals[0][0], intervals[0][1]);

        for (_, v) in intervals.iter().enumerate() {
            // have overlap => extend
            if v[0] <= right {
                right = right.max(v[1]);
            } else { // no overlap => add to result
                result.push(vec![left, right]);
                left = v[0];
                right = v[1];
            }
        }
        // add the last interval
        result.push(vec![left, right]);

        result
    }

    use std::{ops::RangeInclusive, mem::swap, cmp::max};
    fn range_to_vec(range:RangeInclusive<i32>)->Vec<i32> {
        vec![*range.start(),*range.end()]
    }
    pub fn merge3(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals:Vec<RangeInclusive<i32>> = intervals.into_iter().map(|interval|interval[0]..=interval[1]).collect();
        intervals.sort_unstable_by(|interval1,interval2|interval1.start().cmp(interval2.start()));
        let mut intervals = intervals.into_iter();
        let mut output = Vec::new();
        if let Some(first) = intervals.next() {
            let mut current = first;
            for mut interval in intervals {
                if interval.start()<=current.end() {
                    current=*current.start()..=max(*current.end(), *interval.end());
                }else {
                    swap(&mut current, &mut interval);
                    output.push(range_to_vec(interval));
                }
            }
            output.push(range_to_vec(current));
            output
        } else {
            vec![]
        }
    }
}


#[cfg(test)]
mod  test {
    use crate::leet75_l2::a_00056_merge_intervals::a56::{merge, merge2, merge3};

    #[test]
    fn t_00() {
        let inp = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let ans = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(ans, merge(inp));
    }

    #[test]
    fn t_001() {
        let inp = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let ans = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(ans, merge2(inp));
    }

    #[test]
    fn t_002() {
        let inp = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let ans = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(ans, merge3(inp));
    }
}