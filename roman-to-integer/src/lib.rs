use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let chars: Vec<_> = s.chars().collect();
        let roman = {
            let mut map = HashMap::new();
            map.insert('I', 1);
            map.insert('V', 5);
            map.insert('X', 10);
            map.insert('L', 50);
            map.insert('C', 100);
            map.insert('D', 500);
            map.insert('M', 1000);
            map
        };

        for i in (0..s.len()).rev() {
            let val = roman.get(&chars[i]).unwrap();

            if i + 1 < s.len() && val < roman.get(&chars[i + 1]).unwrap() {
                result -= *val;
            } else {
                result += *val;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_int_to_roman() {
        let s = String::from("III");
        assert_eq!(Solution::roman_to_int(s), 3);

        let s = String::from("IV");
        assert_eq!(Solution::roman_to_int(s), 4);

        let s = String::from("IX");
        assert_eq!(Solution::roman_to_int(s), 9);

        let s = String::from("LVIII");
        assert_eq!(Solution::roman_to_int(s), 58);

        let s = String::from("MCMXCIV");
        assert_eq!(Solution::roman_to_int(s), 1994);
    }
}
