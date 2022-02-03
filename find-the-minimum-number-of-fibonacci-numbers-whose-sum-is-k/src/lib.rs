#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_find_min_fibonacci_numbers() {
        assert_eq!(Solution::find_min_fibonacci_numbers(7), 2);
        assert_eq!(Solution::find_min_fibonacci_numbers(10), 2);
        assert_eq!(Solution::find_min_fibonacci_numbers(17), 3);
        assert_eq!(Solution::find_min_fibonacci_numbers(5702920), 5);
    }
}

pub struct Solution {}

impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut k = k;
        let mut len = 2;
        let mut ans = 0;
        let mut fib = vec![1, 1];

        while k > fib[len - 1] {
            fib.push(fib[len - 1] + fib[len - 2]);
            len += 1;
        }

        while k > 0 {
            if k >= fib[len - 1] {
                ans += 1;
                k -= fib[len - 1];
            }
            len -= 1;
        }
        ans
    }
}
