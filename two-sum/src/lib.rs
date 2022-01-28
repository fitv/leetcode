#[cfg(test)]
mod tests {
    #[test]
    fn test_two_sum() {
        let mut result = 0;
        let nums = vec![2, 7, 11, 15];
        let target = 18;

        for i in super::two_sum(&nums, target) {
            result += nums[i];
        }
        assert_eq!(target, result);
    }
}

pub fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<usize> {
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i, j];
            }
        }
    }
    panic!("target not exists");
}
