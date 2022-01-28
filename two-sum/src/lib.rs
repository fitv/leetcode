use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::two_sum;

    #[test]
    fn test_two_sum() {
        let nums = [2, 7, 11, 15];

        assert_eq!(Some((0, 1)), two_sum(&nums, 9));
        assert_eq!(Some((1, 2)), two_sum(&nums, 18));
        assert_eq!(Some((2, 3)), two_sum(&nums, 26));
        assert_eq!(None, two_sum(&nums, 100));
    }
}

pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let map = {
        let mut map = HashMap::new();

        for (i, &val) in nums.iter().enumerate() {
            map.insert(val, i);
        }
        map
    };

    for (val, i) in &map {
        let elem = map.get(&(target - val));

        if elem.is_some() {
            return Some({
                let (x, y) = (*i, *elem.unwrap());

                if x > y {
                    (y, x)
                } else {
                    (x, y)
                }
            });
        }
    }
    None
}
