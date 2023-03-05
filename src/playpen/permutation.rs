/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

pub  mod pm {
    pub  fn  t001(inp:Vec<i32>, profits:Vec<i32>) -> i32{
        let  mut ans =0;
        let n = profits.len();
        let mut x: Vec<(i32, i32)> = (0..n)
            .map(|x|(profits[x], inp[x]))
            .collect();

        println!("{:?}",x);


        ans
    }


}



#[cfg(test)]
mod test{
    use crate::playpen::permutation::pm::t001;
    #[test]
    fn  t_001(){
        let  inp =vec![1,2,3,4,5,7];
        let  profits:Vec<i32> =vec![111,222,333,444,555];
        assert_eq!(0, t001(inp,profits));
    }

}