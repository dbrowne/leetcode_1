pub mod o7 {

    pub fn reverse_bad(x: i32) -> i32 {
        let mut stk: Vec<i32> = Vec::new();
        let mut res = 0;
        if x == i32::MIN {
            return 0;
        }
        if x.abs() < 10 {
            return x;
        }
        let mut res = x;
        loop {
            let mut tmp = res % 10;

            if res.abs() < 10 {
                stk.push(res);
                break;
            }
            stk.push(tmp);
            res = res / 10;
        }

        let pow = stk.len();
        if pow == 10 {
            let t_vec = vec![2, 1, 4, 7, 4, 8, 3, 6, 4, 8];
            for i in 0..4 {
                if stk[i].abs() > t_vec[i] {
                    return 0;
                }
            }
            if stk[0].abs() > 2 {
                return 0;
            }
        }
        let mut ret_val = 0;

        for x in 0..pow as i32 {
            ret_val += i32::pow(10, x as u32) * stk.pop().unwrap();
        }
        if ret_val < 0 && x > 0 {
            return 0;
        }
        ret_val
    }

    pub fn reverse(x: i32) -> i32 {
        let mut rev = 0;
        let mut x_mut = x;
        while x_mut != 0 {
            let pop = x_mut % 10;
            x_mut /= 10;
            if (rev > i32::MAX / 10 || (rev == i32::MAX / 10 && pop > 7)) {
                return 0;
            }
            if (rev < i32::MIN / 10 || (rev == i32::MIN / 10 && pop < -8)) {
                return 0;
            }
            rev = rev * 10 + pop;
        }
        rev
    }
}

#[cfg(test)]
mod test {

    use crate::general_problems::a_0007_reverse_integer::o7::reverse;
    #[test]
    fn test_01() {
        assert_eq!(321, reverse(123));
    }
    #[test]
    fn test_02() {
        assert_eq!(-321, reverse(-123));
    }
    #[test]
    fn test_03() {
        assert_eq!(21, reverse(120));
    }
    #[test]
    fn test_04() {
        assert_eq!(0, reverse(1534236469));
    }
    #[test]
    fn test_05() {
        assert_eq!(0, reverse(-2147483648))
    }
    #[test]
    fn test_06() {
        assert_eq!(0, reverse(1563847412));
    }

    #[test]
    fn test_07() {
        assert_eq!(0, reverse(-1563847412));
    }
    #[test]
    fn test_08() {
        assert_eq!(-2143847412, reverse(-2147483412))
    }
}
