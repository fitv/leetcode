#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums, vec![1, 2]);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![0, 1, 2, 3, 4]);
    }
}

pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut len = 1;

        for i in 1..nums.len() {
            if nums[len - 1] != nums[i] {
                nums[len] = nums[i];
                len += 1;
            }
        }
        unsafe {
            nums.set_len(len);
        }

        len as i32
    }
}
