/*
 * Copyright (c) 2023.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

// A conveyor belt has packages that must be shipped from one port to another within days days.
//
// The ith package on the conveyor belt has a weight of weights[i]. Each day, we load the ship with packages on the conveyor belt (in the order given by weights). We may not load more weight than the maximum weight capacity of the ship.
//
// Return the least weight capacity of the ship that will result in all the packages on the conveyor belt being shipped within days days.
//
//
//
// Example 1:
//
// Input: weights = [1,2,3,4,5,6,7,8,9,10], days = 5
// Output: 15
// Explanation: A ship capacity of 15 is the minimum to ship all the packages in 5 days like this:
// 1st day: 1, 2, 3, 4, 5
// 2nd day: 6, 7
// 3rd day: 8
// 4th day: 9
// 5th day: 10
//
// Note that the cargo must be shipped in the order given, so using a ship of capacity 14 and splitting the packages into parts like (2, 3, 4, 5), (1, 6, 7), (8), (9), (10) is not allowed.
// Example 2:
//
// Input: weights = [3,2,2,4,1,4], days = 3
// Output: 6
// Explanation: A ship capacity of 6 is the minimum to ship all the packages in 3 days like this:
// 1st day: 3, 2
// 2nd day: 2, 4
// 3rd day: 1, 4
// Example 3:
//
// Input: weights = [1,2,3,1,1], days = 4
// Output: 3
// Explanation:
// 1st day: 1
// 2nd day: 2
// 3rd day: 3
// 4th day: 1, 1
//
//
// Constraints:
//
// 1 <= days <= weights.length <= 5 * 104
// 1 <= weights[i] <= 500

pub  mod  a1011{
    pub  fn ship_within_days(weights:Vec<i32>, days: i32)->i32{
        let mut min_c:i32 =0;
        let  mut max_c:i32 =0;

        for weight  in  &weights {
            min_c = min_c.max(*weight);
            max_c += weight;
        }

        while min_c < max_c {
            let  mut payload:i32 = 0;
            let  mut day_count:i32 =0;

            let  mid_p:i32 = (min_c +max_c)/2;

            for weight  in &weights  {
                if payload + *weight >mid_p {
                    day_count +=1;
                    payload =0;
                }
                payload +=*weight;
            }

            if  day_count >= days{
                min_c = mid_p +1;

            }else {
                max_c = mid_p;
            }
        }

        min_c
    }
}


#[cfg(test)]
mod  test{
    use crate::general_problems::a_1011_capacity_to_ship_packages_within_d_days::a1011::ship_within_days;

    #[test]
    fn t_001(){
        let  inp:Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
        let  days:i32 = 5;
        let  ans:i32 = 15;

        assert_eq!(ans,ship_within_days(inp,days) );
    }

    #[test]
    fn t_002(){
        let  inp:Vec<i32> = vec![3,2,2,4,1,4];
        let  days:i32 = 3;
        let  ans:i32 =  6;

        assert_eq!(ans, ship_within_days(inp,days));
    }

    #[test]
    fn t_003(){
        let  inp:Vec<i32> = vec![1,2,3,1,1];
        let  days:i32 = 4;
        let  ans = 3;
        assert_eq!(ans, ship_within_days(inp,days));
    }
}