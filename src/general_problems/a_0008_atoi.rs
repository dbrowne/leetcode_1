pub mod atoi {

    pub fn my_atoi(s: String) -> i32 {
        let mut ans: i64 = 0;
        let mut sign = 1;
        let mut has_num = false;

        for chr in s.chars().into_iter() {
            if !has_num {
                match chr {
                    ' ' => {}
                    '0'..='9' => {
                        has_num = true;
                        ans = ans * 10 + chr.to_digit(10).unwrap() as i64;
                    }
                    '-' => {
                        has_num = true;
                        sign = -1;
                    }
                    '+' => {
                        has_num = true;
                    }
                    _ => return 0,
                }
            } else {
                match chr {
                    '0'..='9' => {
                        ans = ans * 10 + chr.to_digit(10).unwrap() as i64;
                        if ans > i32::MAX as i64 {
                            break;
                        }
                    }
                    _ => break,
                }
            }
        }
        ans *= sign;
        if ans > i32::MAX as i64 {
            return i32::MAX;
        }

        if ans < i32::MIN as i64 {
            return i32::MIN;
        }
        return ans as i32;
    }
}

#[cfg(test)]
mod test {
    use crate::general_problems::a_0008_atoi::atoi::my_atoi;

    #[test]
    fn test_01() {
        assert_eq!(44, my_atoi(String::from("44")));
    }

    #[test]
    fn test_02() {
        assert_eq!(-44, my_atoi(String::from("-44")));
    }

    #[test]
    fn test_03() {
        assert_eq!(-42, my_atoi(String::from("   -42")));
    }

    #[test]
    fn test_4() {
        assert_eq!(4193, my_atoi(String::from("4193 with words")));
    }

    #[test]
    fn test_5() {
        assert_eq!(0, my_atoi(String::from("words and 987")));
    }
}
