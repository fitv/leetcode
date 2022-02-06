pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        let mut values = (0, 0);
        let position = {
            if len % 2 == 0 {
                (len / 2 - 1, len / 2)
            } else {
                (len / 2, len / 2)
            }
        };
        let (mut head, mut i, mut j) = (0, 0, 0);
        let (len1, len2) = (nums1.len(), nums2.len());

        while i < len1 || j < len2 {
            let val;

            if i < len1 && j < len2 {
                if nums1[i] < nums2[j] {
                    val = nums1[i];
                    i += 1
                } else {
                    val = nums2[j];
                    j += 1;
                }
            } else if i < len1 {
                val = nums1[i];
                i += 1;
            } else {
                val = nums2[j];
                j += 1;
            }

            if head == position.0 {
                values.0 = val;
            }
            if head == position.1 {
                values.1 = val;
                break;
            }

            head += 1;
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
