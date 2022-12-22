pub mod day_11 {
    use std::cmp;
    use std::cmp::min;

    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut c_vec = cost.clone();
        c_vec.push(0);
        let mut ctr = c_vec.len() - 3;
        loop {
            c_vec[ctr] += min(c_vec[ctr + 1], c_vec[ctr + 2]);
            if ctr == 0 {
                break;
            }
            ctr -= 1;
        }
        min(c_vec[0], c_vec[1])
    }

    pub  fn  min_cost_climbing_stairs_fast(cost: Vec<i32> )->i32{
        let mut vec = vec![0; cost.len() + 1];
        for i in 2..vec.len() {
            vec[i] += cmp::min(vec[i-1] + cost[i-1], vec[i-2] +cost[i-2]);
        }

        vec[cost.len()]

    }

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut grpp: Vec<Vec<usize>> = vec![vec![1; m as usize]; n as usize];

        for row in 1..n as usize {
            for col in 1..m as usize {
                grpp[row][col] = grpp[row - 1][col] + grpp[row][col - 1];
            }
        }
        grpp[(n - 1) as usize][(m - 1) as usize] as i32
    }
}

#[cfg(test)]
mod test {
    use crate::leetweektwo::day11::day_11::{min_cost_climbing_stairs, unique_paths};

    #[test]
    fn test_01() {
        assert_eq!(15, min_cost_climbing_stairs(vec![10, 15, 20]));
    }

    #[test]
    fn test_02() {
        assert_eq!(3, unique_paths(3, 2));
    }
}
