/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// Suppose LeetCode will start its IPO soon. In order to sell a good price of its shares to Venture Capital, LeetCode would like to work on some projects to increase its capital before the IPO. Since it has limited resources, it can only finish at most k distinct projects before the IPO. Help LeetCode design the best way to maximize its total capital after finishing at most k distinct projects.
//
// You are given n projects where the ith project has a pure profit profits[i] and a minimum capital of capital[i] is needed to start it.
//
// Initially, you have w capital. When you finish a project, you will obtain its pure profit and the profit will be added to your total capital.
//
// Pick a list of at most k distinct projects from given projects to maximize your final capital, and return the final maximized capital.
//
// The answer is guaranteed to fit in a 32-bit signed integer.
//
//
//
// Example 1:
//
// Input: k = 2, w = 0, profits = [1,2,3], capital = [0,1,1]
// Output: 4
// Explanation: Since your initial capital is 0, you can only start the project indexed 0.
// After finishing it you will obtain profit 1 and your capital becomes 1.
// With capital 1, you can either start the project indexed 1 or the project indexed 2.
// Since you can choose at most 2 projects, you need to finish the project indexed 2 to get the maximum capital.
// Therefore, output the final maximized capital, which is 0 + 1 + 3 = 4.
// Example 2:
//
// Input: k = 3, w = 0, profits = [1,2,3], capital = [0,1,2]
// Output: 6
//
//
// Constraints:
//
// 1 <= k <= 105
// 0 <= w <= 109
// n == profits.length
// n == capital.length
// 1 <= n <= 105
// 0 <= profits[i] <= 104
// 0 <= capital[i] <= 109

pub  mod  a_502{
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let  mut ans:i32 = w;
        let  mut cnt:i32 = 0;
        let  mut heap1= BinaryHeap::new();
        let  mut heap2:BinaryHeap<i32>= BinaryHeap::new();

        for i in 0 .. capital.len() {
            heap1.push(Reverse((capital[i],i)));
        }

        while !heap1.is_empty() {
            while let  Some(Reverse((cnt,i))) = heap1.peek() {
                if *cnt > ans {
                    break
                }
                heap2.push(profits[*i]);
                heap1.pop();

            }

            if let Some(money) = heap2.pop() {
                ans += money;
                cnt +=1;
                if cnt == k {
                    break
                }
                
            }else { 
                break
            }
        }

        while let Some(money) = heap2.pop(){
            if  cnt  == k{
                break
            }
            cnt +=1;
            ans += money;
            
        }
        
        ans
    }


    pub fn find_maximized_capital_fast(mut k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        let mut x: Vec<(i32, i32)> = (0..n)
            .map(|x|(profits[x], capital[x]))
            .collect();
        x.sort_unstable_by_key(|t|(*t).1);
        let mut ptr = 0;
        let mut bp = BinaryHeap::new();
        while k > 0 {
            while ptr < n && x[ptr].1 <= w {
                bp.push(x[ptr].0);
                ptr += 1;
            }
            // println!("{:?}", bp);
            if let Some(tp) = bp.pop() {
                w += tp;
                k -= 1;
            } else {
                break;
            }
        }
        w
    }
}


#[cfg(test)]
mod  test{
    use crate::general_problems::a_502_IPO::a_502::find_maximized_capital;

    #[test]
    fn t_001(){
        let  k =2;
        let w = 0;
        let profits: Vec<i32> = vec![1, 2, 3];
        let capital: Vec<i32> = vec![0, 1, 1];
        let  ans = 4;

        assert_eq!(ans, find_maximized_capital(k, w, profits,capital));
    }

    #[test]
    fn t_002(){
        let  k = 3;
        let w = 0;
        let profits: Vec<i32> = vec![1, 2, 3];
        let capitol: Vec<i32> = vec![0, 1, 2];
        let ans = 6;

        assert_eq!(ans, find_maximized_capital(k,w,profits,capitol));
    }
}