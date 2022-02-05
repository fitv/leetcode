// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
impl ListNode {
    fn append(&mut self, val: i32) {
        match &mut self.next {
            None => {
                self.next = Some(Box::new(Self::new(val)));
            }
            Some(ref mut x) => x.append(val),
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        let mut n1 = list1.unwrap();
        let mut n2 = list2.unwrap();

        if n1.val < n2.val {
            n1.next = Self::merge_two_lists(n1.next, Some(n2));
            return Some(n1);
        } else {
            n2.next = Self::merge_two_lists(Some(n1), n2.next);
            return Some(n2);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    #[test]
    fn test_merge_two_lists() {
        let mut n1 = ListNode::new(1);
        n1.append(2);
        n1.append(4);
        let l1 = Some(Box::new(n1));

        let mut n2 = ListNode::new(1);
        n2.append(3);
        n2.append(4);
        let l2 = Some(Box::new(n2));

        let mut result = Solution::merge_two_lists(l1, l2);
        let mut nums = vec![];

        loop {
            if result.is_none() {
                break;
            }
            let node = result.unwrap();
            nums.push(node.val);
            result = node.next
        }

        assert_eq!(nums, vec![1, 1, 2, 3, 4, 4]);
    }
}
