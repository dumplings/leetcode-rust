use crate::models::linked_list::ListNode;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        if head.is_none() {
            return None;
        }

        let mut current = head.as_mut().unwrap();

        while let Some(next_node) = current.next.as_mut() {
            if current.val == next_node.val {
                current.next = next_node.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::linked_list::vec_to_list;
    #[test]
    pub fn test_delete_duplicates() {
        let case_1 = vec_to_list(vec![1, 1, 2]);
        let res_1 = vec_to_list(vec![1, 2]);
        let case_2 = vec_to_list(vec![1, 1, 2, 3, 3]);
        let res_2 = vec_to_list(vec![1, 2, 3]);

        assert_eq!(Solution::delete_duplicates(case_1), res_1);
        assert_eq!(Solution::delete_duplicates(case_2), res_2);
    }
}
