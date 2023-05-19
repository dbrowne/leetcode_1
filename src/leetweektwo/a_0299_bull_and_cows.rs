/*
 * Copyright (c) 2022.
 * Dwight Browne
 * dwight@dwightjbrowne.com
 */

/*
You are playing the Bulls and Cows game with your friend.

You write down a secret number and ask your friend to guess what the number is. When your friend makes a guess, you provide a hint with the following info:

The number of "bulls", which are digits in the guess that are in the correct position.
The number of "cows", which are digits in the guess that are in your secret number but are located in the wrong position. Specifically, the non-bull digits in the guess that could be rearranged such that they become bulls.
Given the secret number secret and your friend's guess guess, return the hint for your friend's guess.

The hint should be formatted as "xAyB", where x is the number of bulls and y is the number of cows. Note that both secret and guess may contain duplicate digits.



Example 1:

Input: secret = "1807", guess = "7810"
Output: "1A3B"
Explanation: Bulls are connected with a '|' and cows are underlined:
"1807"
  |
"7810"
Example 2:

Input: secret = "1123", guess = "0111"
Output: "1A1B"
Explanation: Bulls are connected with a '|' and cows are underlined:
"1123"        "1123"
  |      or     |
"0111"        "0111"
Note that only one of the two unmatched 1s is counted as a cow since the non-bull digits can only be rearranged to allow one 1 to be a bull.


Constraints:

1 <= secret.length, guess.length <= 1000
secret.length == guess.length
secret and guess consist of digits only.
 */

pub mod day14 {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bull_count = 0;
        let mut cow_count = 0;
        let mut secret_i = secret.chars();
        let mut guess_i = guess.chars();
        let mut bull_b = vec![0; 10];
        let mut missed = vec![];

        while let (Some(sec), Some(gss)) = (secret_i.next(), guess_i.next()) {
            if sec == gss {
                bull_count += 1;
            } else {
                bull_b[(sec as i32 - 0x30) as usize] +=1;
                missed.push((gss as i32 - 0x30) as usize);

            }
        }

        for &idx in missed.iter()  {
            if bull_b[idx] >0 {
                cow_count+=1;
                bull_b[idx] -=1;
            }

        }
        format!("{}A{}B",bull_count, cow_count)
    }

    pub fn get_hint_fast(secret: String, guess: String) -> String {
        let mut bulls = 0;
        let mut cows = 0;
        let mut count = vec![0; 10];
        for (c1, c2) in secret.chars().zip(guess.chars()) {
            if c1 == c2 {
                bulls += 1;
            } else {
                let n1 = (c1 as u8 - b'0') as usize;
                let n2 = (c2 as u8 - b'0') as usize;
                if count[n1] < 0 {
                    cows += 1;
                }
                if count[n2] > 0 {
                    cows += 1;
                }
                count[n1] += 1;
                count[n2] -= 1;
            }
        }
        format!("{}A{}B", bulls, cows)
    }

}


#[cfg(test)]
mod test {
    use crate::leetweektwo::a_0299_bull_and_cows::day14::get_hint;

    #[test]
    fn test_01() {

        assert_eq!(get_hint(String::from("1807"), String::from("7810")),String::from("1A3B"));
    }

    #[test]
    fn test_02(){
        assert_eq!(get_hint(String::from("1123"),String::from("0111")),String::from("1A1B") );
    }


}