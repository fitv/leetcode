pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut len = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                len += 1;
                nums[len - 1] = nums[i];
            }
        }
        len as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_remove_element() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut nums, 3), 2);
        assert_eq!(nums.get(0..2).unwrap(), vec![2, 2]);

        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut nums, 2), 5);
        assert_eq!(nums.get(0..5).unwrap(), vec![0, 1, 3, 0, 4]);
    }
}
