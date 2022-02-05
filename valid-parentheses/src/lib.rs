pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for ch in s.chars() {
            match ch {
                '(' => {
                    stack.push(')');
                }
                '[' => {
                    stack.push(']');
                }
                '{' => {
                    stack.push('}');
                }
                _ => {
                    if stack.pop() != Some(ch) {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_is_valid() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
        assert_eq!(Solution::is_valid("(])".to_string()), false);
    }
}
