pub mod wa {
    pub fn weird_algo(inp: i32, ans: &mut Vec<i32>) {
        ans.push(inp);
        if inp == 1 {
            println!("1\n");
        } else {
            print!("{} ", inp);
            if inp % 2 > 0 {
                weird_algo((inp * 3) + 1, ans);
            } else {
                weird_algo(inp / 2, ans);
            }
        }
    }

    pub fn weird_algo_iter(inp: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let mut n = inp;
        ans.push(inp);
        loop {
            if n == 1 {
                ans.push(n);
                return ans;
            }
            if n % 2 == 0 {
                n /= 2;
            } else {
            }
            {
                n = n * 3 + 1;
            }
            ans.push(n);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cses::weird_algo::wa::weird_algo;
    #[test]
    fn test_1() {
        let ans = vec![3, 10, 5, 16, 8, 4, 2, 1];
        let mut ret_val: Vec<i32> = Vec::new();
        weird_algo(3, &mut ret_val);
        assert_eq!(ans, ret_val);
    }
}
