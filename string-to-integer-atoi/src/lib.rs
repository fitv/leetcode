#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_my_atoi() {
        let s = String::from("");
        assert_eq!(Solution::my_atoi(s), 0);

        let s = String::from("42");
        assert_eq!(Solution::my_atoi(s), 42);

        let s = String::from("   -42");
        assert_eq!(Solution::my_atoi(s), -42);

        let s = String::from("4193 with words");
        assert_eq!(Solution::my_atoi(s), 4193);

        let s = String::from("words and 987");
        assert_eq!(Solution::my_atoi(s), 0);

        let s = String::from("-91283472332");
        assert_eq!(Solution::my_atoi(s), i32::MIN);
    }
}

pub struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut sum: i32 = 0;
        let mut negative = false;
        let chars: Vec<_> = s.trim().chars().collect();

        match chars.first() {
            Some(v) => match v {
                '-' => {
                    negative = true;
                }
                '+' => {}
                x @ '0'..='9' => {
                    sum = Self::char_to_digit(x);
                }
                _ => return 0,
            },
            None => return 0,
        }

        for i in 1..chars.len() {
            match chars.get(i).unwrap() {
                x @ '0'..='9' => {
                    let x = Self::char_to_digit(x);

                    if sum.checked_mul(10).is_none() {
                        return if negative { i32::MIN } else { i32::MAX };
                    }
                    sum *= 10;

                    if sum.checked_add(x).is_none() {
                        return if negative { i32::MIN } else { i32::MAX };
                    }
                    sum += x;
                }
                _ => break,
            }
        }

        return if negative { -sum } else { sum };
    }

    fn char_to_digit(c: &char) -> i32 {
        return match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => 0,
        };
    }
}
