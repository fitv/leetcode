use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_uncommon_from_sentences() {
        let s1 = String::from("this apple is sweet");
        let s2 = String::from("this apple is sour");
        let result = Solution::uncommon_from_sentences(s1, s2);
        assert!(result == vec!["sweet", "sour"] || result == vec!["sour", "sweet"]);

        let s1 = String::from("apple apple");
        let s2 = String::from("banana");
        assert_eq!(Solution::uncommon_from_sentences(s1, s2), vec!["banana"]);
    }
}

pub struct Solution {}

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let s = s1 + " " + s2.as_str();
        let mut words = vec![];
        let mut map = HashMap::new();

        for word in s.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        for (word, count) in &map {
            if *count == 1 {
                words.push((*word).to_string());
            }
        }
        words
    }
}
