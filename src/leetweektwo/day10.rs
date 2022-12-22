pub mod day_10 {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let (mut prev, mut curr) = (1, 2);
        for i in 2..n {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use crate::leetweektwo::day10::day_10::climb_stairs;

    #[test]
    fn test01() {
        assert_eq!(climb_stairs(5), 8);
    }
}
