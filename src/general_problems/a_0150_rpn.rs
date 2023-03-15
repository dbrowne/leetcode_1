pub mod a150 {
    pub fn eval_rpn(tokens: Vec<&str>) -> i32 {
        let mut stk: Vec<i32> = Vec::new();
        for tok in tokens {
            if let Ok(val) = tok.parse::<i32>() {
                stk.push(val);
            } else {
                let rtok = stk.pop().unwrap();
                let ltok = stk.pop().unwrap();
                match &tok as &str {
                    "+" => stk.push(ltok + rtok),
                    "-" => stk.push(ltok - rtok),
                    "*" => stk.push(ltok * rtok),
                    "/" => stk.push(ltok / rtok),
                    _ => unreachable!(),
                }
            }
        }

        stk.pop().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::general_problems::a_0150_rpn::a150::eval_rpn;

    #[test]
    fn test_001() {
        let toks = vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ];
        assert_eq!(eval_rpn(toks), 22);
    }
}

// use std::ops::{Add, Sub, Mul, Div};
//
// fn eval(stack: &mut Vec<i32>, op: impl FnOnce(i32, i32) -> i32) {
//     let b = stack.pop().unwrap();
//     let a = stack.pop().unwrap();
//     stack.push(op(a, b));
// }
//
// impl Solution {
//     pub fn eval_rpn(tokens: Vec<String>) -> i32 {
//         let mut stack = Vec::new();
//
//         for token in tokens {
//             match token.as_str() {
//                 "+" => eval(&mut stack, Add::add),
//                 "-" => eval(&mut stack, Sub::sub),
//                 "*" => eval(&mut stack, Mul::mul),
//                 "/" => eval(&mut stack, Div::div),
//                 _ => stack.push(token.parse().unwrap()),
//             }
//         }
//
//         stack[0]
//     }
