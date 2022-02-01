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
        assert_eq!(Solution::my_atoi(s), -2147483648);
    }
}

pub struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut sum: i32 = 0;
        let mut negative = false;
        let max = i32::MAX;
        let max_negative = i32::MIN;
        let chars: Vec<_> = s.trim().chars().collect();

        match chars.first() {
            Some(v) => match v {
                '-' => {
                    negative = true;
                }
                '+' => {}
                x @ '0'..='9' => {
                    sum = x.to_digit(10).unwrap() as i32;
                }
                _ => return 0,
            },
            None => return 0,
        }

        for i in 1..chars.len() {
            match chars.get(i).unwrap() {
                x @ '0'..='9' => {
                    let x = x.to_digit(10).unwrap() as i32;

                    if sum.checked_mul(10).is_none() {
                        return if negative { max_negative } else { max };
                    }
                    sum *= 10;

                    if sum.checked_add(x).is_none() {
                        return if negative { max_negative } else { max };
                    }
                    sum += x;
                }
                _ => break,
            }
        }

        return if negative { -sum } else { sum };
    }
}
