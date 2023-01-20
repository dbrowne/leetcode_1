/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */


// You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent the start and the end of the ith interval and intervals is sorted in ascending order by starti.
// You are also given an interval newInterval = [start, end] that represents the start and end of another interval.
//
// Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).
//
// Return intervals after the insertion.
//
//
//
// Example 1:
//
// Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
// Output: [[1,5],[6,9]]
// Example 2:
//
// Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
// Output: [[1,2],[3,10],[12,16]]
// Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
//
//
// Constraints:
//
// 0 <= intervals.length <= 104
// intervals[i].length == 2
// 0 <= starti <= endi <= 105
// intervals is sorted by starti in ascending order.
// newInterval.length == 2
// 0 <= start <= end <= 105

mod  a57{



    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::{min, max};
        let mut less = vec![];
        let mut more = vec![];
        let mut start = new_interval[0];
        let mut end = new_interval[1];

        for curr in intervals{
            if curr[1] < new_interval[0]{
                less.push(curr);
            }
            else if curr[0]>new_interval[1]{
                more.push(curr);
            }
            else {
                start = min(curr[0], start);
                end = max(curr[1], end);
            }
        }
        less.push(vec![start, end]);
        less.append(&mut more);
        less
    }
}


#[cfg(test)]
mod  test{
    use crate::leet75_L2::a_00057_insert_interval::a57::insert;
    #[test]
    fn t_00(){
        let  inp = vec![vec![1,3],vec![6,9]];
        let  ins = vec![2,5] ;

        let  ans = vec![vec![1,5],vec![6,9]];

        assert_eq!(ans,insert(inp,ins) );
    }


}