use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_longest_nice_substring() {
        let s = String::from("c");
        assert_eq!(Solution::longest_nice_substring(s), "");

        let s = String::from("Bb");
        assert_eq!(Solution::longest_nice_substring(s), "Bb");

        let s = String::from("dDzeE");
        assert_eq!(Solution::longest_nice_substring(s), "dD");

        let s = String::from("abABB");
        assert_eq!(Solution::longest_nice_substring(s), "abABB");

        let s = String::from("YazaAay");
        assert_eq!(Solution::longest_nice_substring(s), "aAa");
    }
}

pub struct Solution {}

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let mut max = 0;
        let mut result = String::new();
        let chars: Vec<_> = s.chars().collect();

        let mut find_longest = |start, end| {
            let mut m1 = HashMap::new();
            let mut m2 = HashMap::new();

            for c in chars.get(start..=end).unwrap() {
                m1.insert(c, 0);
                m2.insert(c.to_lowercase().to_string(), 0);
            }

            if m1.len() == m2.len() * 2 && (end - start + 1) > max {
                max = end - start + 1;
                result = chars.get(start..=end).unwrap().iter().collect();
            }
        };

        for i in 0..chars.len() {
            for j in (i + 1)..chars.len() {
                find_longest(i, j);
            }
        }
        result
    }
}
