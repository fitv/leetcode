use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut map = HashMap::new();

        for i in 0..nums.len() {
            let val1 = map.get(&(nums[i] - k));
            let val2 = map.get(&(nums[i] + k));

            if val1.is_some() {
                ans += *val1.unwrap();
            }
            if val2.is_some() {
                ans += *val2.unwrap();
            }

            let count = map.entry(nums[i]).or_insert(0);
            *count += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_count_k_difference() {
        assert_eq!(Solution::count_k_difference(vec![1, 2, 2, 1], 1), 4);
        assert_eq!(Solution::count_k_difference(vec![1, 3], 1), 0);
        assert_eq!(Solution::count_k_difference(vec![3, 2, 1, 5, 4], 2), 3);
    }
}
