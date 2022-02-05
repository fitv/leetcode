use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let mut nums = nums;
        let mut ans = 0;
        let mut diff = i32::MAX;

        if len < 3 {
            return 0;
        }

        nums.sort();

        for i in 0..len - 2 {
            let mut left = i + 1;
            let mut right = len - 1;

            while right > left {
                let sum = nums[i] + nums[left] + nums[right];

                match sum.cmp(&target) {
                    cmp::Ordering::Greater => {
                        if sum - target < diff {
                            diff = sum - target;
                            ans = sum;
                        }
                        right -= 1;
                    }
                    cmp::Ordering::Less => {
                        if target - sum < diff {
                            diff = target - sum;
                            ans = sum;
                        }
                        left += 1;
                    }
                    cmp::Ordering::Equal => return target,
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_three_sum() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 1], 0), 3);
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 0], -100), 2);
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 5, 10, 11], 12), 13);
    }
}
