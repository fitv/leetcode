#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(-101), false);
        assert_eq!(Solution::is_palindrome(90000009), true);
    }
}

pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut y = x;
        let mut sum = 0;

        while y != 0 {
            sum = sum * 10 + (y % 10);
            y /= 10;
        }
        sum == x
    }
}
