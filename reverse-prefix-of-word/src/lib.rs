#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_reverse_prefix() {
        let s = String::from("abcdefd");
        assert_eq!(Solution::reverse_prefix(s, 'd'), "dcbaefd");

        let s = String::from("xyxzxe");
        assert_eq!(Solution::reverse_prefix(s, 'z'), "zxyxxe");

        let s = String::from("abcd");
        assert_eq!(Solution::reverse_prefix(s, 'z'), "abcd");
    }
}

pub struct Solution {}

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let chars: Vec<_> = word.chars().collect();

        for i in 0..chars.len() {
            if chars[i] == ch {
                return format!(
                    "{}{}",
                    chars.get(0..=i).unwrap().iter().rev().collect::<String>(),
                    chars
                        .get((i + 1)..chars.len())
                        .unwrap()
                        .iter()
                        .collect::<String>()
                        .as_str()
                );
            }
        }
        word
    }
}
