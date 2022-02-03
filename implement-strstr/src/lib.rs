#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_str_str() {
        let haystack = String::from("hello");
        let needle = String::from("ll");
        assert_eq!(Solution::str_str(haystack, needle), 2);

        let haystack = String::from("aaaaa");
        let needle = String::from("bba");
        assert_eq!(Solution::str_str(haystack, needle), -1);

        let haystack = String::from("");
        let needle = String::from("");
        assert_eq!(Solution::str_str(haystack, needle), 0);
    }
}

pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let len = haystack.len();
        let width = needle.len();

        if width == 0 {
            return 0;
        }
        if width > len {
            return -1;
        }

        let mut next = vec![0; width];
        let haystack: Vec<_> = haystack.chars().collect();
        let needle: Vec<_> = needle.chars().collect();

        let mut j = 0;

        for i in 1..width {
            while j > 0 && needle[i] != needle[j] {
                j = next[j - 1];
            }
            if needle[i] == needle[j] {
                j += 1;
            }
            next[i] = j;
        }

        let mut j = 0;

        for i in 0..len {
            while j > 0 && haystack[i] != needle[j] {
                j = next[j - 1];
            }
            if haystack[i] == needle[j] {
                j += 1;
            }
            if j == width {
                return (i - width + 1) as i32;
            }
        }
        -1
    }
}
