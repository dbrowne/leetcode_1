/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

pub  mod pair {
    pub  fn show_pairs(inp:Vec<i32>) ->Vec<Vec<i32>>{  // a function taking an array input returning an array of arrays
        const MAX_VAL:i32 = 100;
        let  mut counts:Vec<i32>=vec![0;MAX_VAL as usize];  //initialize an array of elements representing the
                                                   // universe of numbers in the input stream
        let mut ret_vec:Vec<Vec<i32>> = vec![];
        for i in  inp.iter(){
            let diff = (MAX_VAL -i) as usize;  // this is the other number we are looking for
            let  i = *i as usize;
            counts[i] +=1;
            if counts[i]>0 && counts[diff] >0{ // if we encountered the numbers then we have a pair
                ret_vec.push(vec![diff as i32, i as i32]);  // push it out onto the return array of arrays
                counts[i] -=1;  // decrement the  count of the occurrence so we do not double count simpler than hashtable implementation
                counts[diff] -=1;
            }
        }
    if ret_vec.len()==0 {
        ret_vec.push(vec![]);
    }
    ret_vec       // rust does not require the return keyword
    }
}


#[cfg(test)]
mod test{
    use crate::general_problems::pairs::pair::show_pairs;

    #[test]
    fn test_01(){
        let inp = vec![1,98,99];
        let t_val :Vec<Vec<i32>> = vec![vec![1,99]];
        assert_eq!(t_val,show_pairs(inp));
    }

    #[test]
    fn test_02(){
        let  inp:Vec<i32> = vec![95,5,95,5];
        let  result:Vec<Vec<i32>> = vec![vec![95,5],vec![95,5]];
        assert_eq!(result,show_pairs(inp))
    }

    #[test]
    fn test_03(){
        let  result:Vec<Vec<i32>>= vec![vec![]];
        let  inp =vec![3,10,1];
        assert_eq!(result,show_pairs(inp) );
    }
}