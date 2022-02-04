use std::cmp;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_three_sum() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
}

pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut nums = nums;
        let mut result = vec![];

        if len < 3 {
            return result;
        }

        nums.sort();

        for i in 0..len {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = len - 1;

            while right > left {
                match (nums[i] + nums[left] + nums[right]).cmp(&0) {
                    cmp::Ordering::Greater => {
                        right -= 1;
                    }
                    cmp::Ordering::Less => {
                        left += 1;
                    }
                    cmp::Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);

                        while right > left && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        while right > left && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        left += 1;
                        right -= 1;
                    }
                }
            }
        }
        result
    }
}
