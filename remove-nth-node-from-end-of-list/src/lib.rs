#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    #[test]
    fn test_remove_nth_from_end() {
        let mut n1 = ListNode::new(1);
        n1.append(2);
        n1.append(3);
        n1.append(4);
        n1.append(5);
        let l1 = Some(Box::new(n1));

        let mut result = Solution::remove_nth_from_end(l1, 2);
        let mut nums = vec![];

        loop {
            if result.is_none() {
                break;
            }
            let node = result.unwrap();
            nums.push(node.val);
            result = node.next
        }

        assert_eq!(nums, vec![1, 2, 3, 5]);
    }
}

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut slow = &mut head;
        let mut fast = &slow.clone();

        for _ in 0..n {
            fast = &fast.as_ref().unwrap().next;
        }

        if fast.as_ref().is_none() {
            return head.unwrap().next;
        }

        while fast.as_ref().unwrap().next.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }
        slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.as_mut().unwrap().next.clone();

        head
    }
}
