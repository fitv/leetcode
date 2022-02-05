use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            area = cmp::max(
                area,
                ((right - left) as i32) * (cmp::min(height[right], height[left])),
            );

            if height[right] > height[left] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        area
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
