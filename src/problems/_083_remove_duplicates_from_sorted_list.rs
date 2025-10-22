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
                todo!("这段代码有问题");
                // current = current.next.as_mut().unwrap();
                current = next_node;
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_delete_duplicates() {}
}
