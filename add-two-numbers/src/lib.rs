#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    #[test]
    fn test_add_two_numbers() {
        let mut n1 = ListNode::new(9);
        n1.append(9);
        n1.append(9);
        n1.append(9);
        n1.append(9);
        n1.append(9);
        n1.append(9);
        let l1 = Some(Box::new(n1));

        let mut n2 = ListNode::new(9);
        n2.append(9);
        n2.append(9);
        n2.append(9);
        let l2 = Some(Box::new(n2));

        let result = Solution::add_two_numbers(l1, l2);
        let mut nums = vec![];
        let mut node = result.unwrap();

        loop {
            nums.push(node.val);

            if node.next.is_none() {
                break;
            }
            node = node.next.unwrap();
        }

        assert_eq!(nums, vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut done1 = false;
        let mut done2 = false;
        let mut n1 = l1.unwrap();
        let mut n2 = l2.unwrap();
        let mut node = ListNode::new(0);

        loop {
            if done1 && done2 && carry == 0 {
                return Some(node.next.unwrap());
            }

            let v1 = if done1 { 0 } else { n1.val };
            let v2 = if done2 { 0 } else { n2.val };

            node.append((v1 + v2 + carry) % 10);
            carry = (v1 + v2 + carry) / 10;

            if n1.next.is_some() {
                n1 = n1.next.unwrap();
            } else {
                done1 = true;
            }
            if n2.next.is_some() {
                n2 = n2.next.unwrap();
            } else {
                done2 = true;
            }
        }
    }
}
