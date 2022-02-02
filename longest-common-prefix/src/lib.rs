#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec!["flower", "flow", "flight"]
            .iter()
            .map(|&str| str.to_string())
            .collect();
        assert_eq!(Solution::longest_common_prefix(strs), "fl");

        let strs = vec!["dog", "racecar", "car"]
            .iter()
            .map(|&str| str.to_string())
            .collect();
        assert_eq!(Solution::longest_common_prefix(strs), "");
    }
}

pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut word = strs[0].clone();

        for i in 1..strs.len() {
            while !strs[i].starts_with(&word) {
                if word.pop().is_none() {
                    return word;
                }
            }
        }
        word
    }
}
