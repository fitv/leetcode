pub struct Solution {}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut num = num;
        let mut times = 0;

        while num != 0 {
            if num % 2 == 0 {
                num >>= 1;
            } else {
                num -= 1;
            }
            times += 1;
        }
        times
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_number_of_steps() {
        assert_eq!(Solution::number_of_steps(8), 4);
        assert_eq!(Solution::number_of_steps(14), 6);
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}
