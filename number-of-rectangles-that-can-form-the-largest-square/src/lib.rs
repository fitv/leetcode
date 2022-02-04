use std::cmp;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_count_good_rectangles() {
        let rectangles = vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]];
        assert_eq!(Solution::count_good_rectangles(rectangles), 3);

        let rectangles = vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]];
        assert_eq!(Solution::count_good_rectangles(rectangles), 3);
    }
}

pub struct Solution {}

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut nums = 0;

        for rectangle in rectangles.iter() {
            let side = cmp::min(rectangle[0], rectangle[1]);

            match side.cmp(&max) {
                cmp::Ordering::Equal => {
                    nums += 1;
                }
                cmp::Ordering::Greater => {
                    max = side;
                    nums = 1;
                }
                _ => {}
            }
        }
        nums
    }
}
