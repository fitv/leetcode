use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::two_sum;

    #[test]
    fn test_two_sum() {
        let nums = [2, 7, 11, 15];

        assert!(Some((0, 1)) == two_sum(&nums, 9) || Some((1, 0)) == two_sum(&nums, 9));
        assert!(Some((1, 2)) == two_sum(&nums, 18) || Some((2, 1)) == two_sum(&nums, 18));
        assert!(Some((2, 3)) == two_sum(&nums, 26) || Some((3, 2)) == two_sum(&nums, 26));
        assert_eq!(None, two_sum(&nums, 100));
    }
}

pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();

    for i in 0..nums.len() {
        let elem = map.get(&(target - nums[i]));

        if elem.is_some() {
            return Some((i, *elem.unwrap()));
        }
        map.insert(nums[i], i);
    }
    None
}
