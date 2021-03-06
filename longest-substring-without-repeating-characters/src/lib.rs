use std::cmp;
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut len = 0;
        let mut cursor = 0;
        let mut map = HashMap::new();
        let chars: Vec<_> = s.chars().collect();

        for i in 0..s.len() {
            let idx = map.get(&chars[i]);

            if idx.is_some() {
                cursor = cmp::max(cursor, *idx.unwrap() + 1);
            }
            len = cmp::max(len, i - cursor + 1);

            map.insert(chars[i], i);
        }
        return len as i32;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_length_of_longest_substring() {
        let s = String::from("pwwkew");
        assert_eq!(Solution::length_of_longest_substring(s), 3);

        let s = String::from("aaaa");
        assert_eq!(Solution::length_of_longest_substring(s), 1);

        let s = String::from("dvdf");
        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }
}
