pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::is_match_regex(&s.chars().collect(), &p.chars().collect())
    }

    fn is_match_regex(s: &Vec<char>, p: &Vec<char>) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }
        let first_match = !s.is_empty() && (s[0] == p[0] || p[0] == '.');

        if p.len() >= 2 && p[1] == '*' {
            Self::is_match_regex(s, &p[2..].to_vec())
                || (first_match && Self::is_match_regex(&s[1..].to_vec(), p))
        } else {
            first_match && Self::is_match_regex(&s[1..].to_vec(), &p[1..].to_vec())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_is_match() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("aa".to_string(), ".*".to_string()), true);
        assert_eq!(
            Solution::is_match("aaa".to_string(), "ab*ac*a".to_string()),
            true
        );
    }
}
