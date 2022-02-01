#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}

pub struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut result = String::new();
        let roman = vec![
            (1, "I"),
            (4, "IV"),
            (5, "V"),
            (9, "IX"),
            (10, "X"),
            (40, "XL"),
            (50, "L"),
            (90, "XC"),
            (100, "C"),
            (400, "CD"),
            (500, "D"),
            (900, "CM"),
            (1000, "M"),
        ];

        for i in (0..roman.len()).rev() {
            while num >= roman[i].0 {
                result.push_str(roman[i].1);
                num -= roman[i].0;
            }
        }
        result
    }
}
