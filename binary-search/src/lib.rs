use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() || nums[0] > target || nums[nums.len() - 1] < target {
            return -1;
        }

        let mut min = 0;
        let mut max = nums.len() - 1;
        let mut mid;

        while min <= max {
            mid = min + (max - min) / 2;

            match nums[mid].cmp(&target) {
                cmp::Ordering::Equal => return mid as i32,
                cmp::Ordering::Greater => max = mid - 1,
                cmp::Ordering::Less => min = mid + 1,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::search(vec![5], -5), -1);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
