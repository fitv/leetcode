pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        let len = nums1.len() + nums2.len();
        let mut tail = len - 1;
        let mut values = (0, 0);
        let position = {
            if len % 2 == 0 {
                (len / 2 - 1, len / 2)
            } else {
                (len / 2, len / 2)
            }
        };

        while tail >= position.0 {
            let val;

            if nums1.len() > 0 && nums2.len() > 0 {
                if nums1.last() > nums2.last() {
                    val = nums1.pop().unwrap();
                } else {
                    val = nums2.pop().unwrap();
                }
            } else if nums1.len() > 0 {
                val = nums1.pop().unwrap();
            } else {
                val = nums2.pop().unwrap();
            }

            if tail == position.0 {
                values.0 = val;
            }
            if tail == position.1 {
                values.1 = val;
            }

            if tail == 0 {
                break;
            }
            tail -= 1;
        }

        ((values.0 + values.1) as f64) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_find_median_sorted_arrays() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);

        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);

        let nums1 = vec![0, 0];
        let nums2 = vec![0, 0];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 0.0);

        let nums1 = vec![];
        let nums2 = vec![1];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 1.0);

        let nums1 = vec![2];
        let nums2 = vec![];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);
    }
}
