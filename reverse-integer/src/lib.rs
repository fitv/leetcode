pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut y = x;
        let mut sum: i32 = 0;

        while y != 0 {
            if sum.checked_mul(10).is_none() {
                return 0;
            }
            sum *= 10;

            if sum.checked_add(y % 10).is_none() {
                return 0;
            }
            sum += y % 10;

            y /= 10;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(1534236469), 0);
        assert_eq!(Solution::reverse(-1534236469), 0);
    }
}
