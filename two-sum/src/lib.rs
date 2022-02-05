use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let val = map.get(&(target - num));

            if val.is_some() {
                return vec![i as i32, *val.unwrap() as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let result = Solution::two_sum(nums, 18);
        assert!(result == vec![1, 2] || result == vec![2, 1]);

        let nums = vec![3, 3];
        let result = Solution::two_sum(nums, 6);
        assert!(result == vec![0, 1] || result == vec![1, 0]);

        let nums = vec![1, 3, 5];
        let result = Solution::two_sum(nums, 10);
        assert_eq!(result, vec![]);
    }
}
