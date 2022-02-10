pub struct Solution {}

impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = vec![];

        for i in 1..n {
            for j in i + 1..=n {
                if Self::gcd(i, j) == 1 {
                    ans.push(format!("{}/{}", i, j));
                }
            }
        }
        ans
    }

    fn gcd(x: i32, y: i32) -> i32 {
        return if y == 0 { x } else { Self::gcd(y, x % y) };
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_simplified_fractions() {
        let expect: Vec<String> = Vec::new();
        assert_eq!(Solution::simplified_fractions(1), expect);

        let expect = vec!["1/2"];
        assert_eq!(Solution::simplified_fractions(2), expect);

        let expect = vec!["1/2", "1/3", "2/3"];
        assert_eq!(Solution::simplified_fractions(3), expect);

        let expect = vec!["1/2", "1/3", "1/4", "2/3", "3/4"];
        assert_eq!(Solution::simplified_fractions(4), expect);
    }
}
