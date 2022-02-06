use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut map = HashMap::new();

        for i in 0..nums.len() {
            let count = map.entry(nums[i]).or_insert(0);

            match *count {
                0 => sum += nums[i],
                1 => sum -= nums[i],
                _ => {}
            }
            *count += 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_sum_of_unique() {
        assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 2]), 4);
        assert_eq!(Solution::sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
        assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
    }
}
