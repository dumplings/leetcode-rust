use crate::models::linked_list::{ListNode, vec_to_list};

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;
        let mut l1 = list1;
        let mut l2 = list2;

        while l1.is_some() && l2.is_some() {
            let l1_val = l1.as_ref().unwrap().val;
            let l2_val = l2.as_ref().unwrap().val;
            if l1_val <= l2_val {
                let next = l1.as_mut().unwrap().next.take();
                cur.next = l1;
                l1 = next;
            } else {
                let next = l2.as_mut().unwrap().next.take();
                cur.next = l2;
                l2 = next;
            }
            cur = cur.next.as_mut().unwrap();
        }
        if l1.is_some() {
            cur.next = l1;
        } else {
            cur.next = l2;
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_two_lists() {
        let l1 = vec![1, 2, 4];
        let l2 = vec![1, 3, 4];
        assert_eq!(
            Solution::merge_two_lists(vec_to_list(l1), vec_to_list(l2)),
            vec_to_list(vec![1, 1, 2, 3, 4, 4])
        );
        assert_eq!(
            Solution::merge_two_lists(vec_to_list(vec![]), vec_to_list(vec![])),
            vec_to_list(vec![])
        );
        assert_eq!(
            Solution::merge_two_lists(vec_to_list(vec![]), vec_to_list(vec![0])),
            vec_to_list(vec![0])
        );
    }
}
