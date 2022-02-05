pub struct Solution {}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            return 0;
        }
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let negative = dividend ^ divisor < 0;
        let mut x = (dividend as i64).abs();
        let y = (divisor as i64).abs();
        let mut ans = 0;

        for i in (0..32).rev() {
            if (x >> i) >= y {
                ans += 1 << i;
                x -= y << i;
            }
        }
        return if negative { -ans } else { ans };
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_divide() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(Solution::divide(-100, 3), -33);
        assert_eq!(Solution::divide(i32::MIN, 2), -1073741824);
        assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
    }
}
