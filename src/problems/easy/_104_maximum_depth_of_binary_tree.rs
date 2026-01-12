use crate::utils::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    // DFS
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left_depth = Self::max_depth(node.borrow().left.clone());
                let right_depth = Self::max_depth(node.borrow().right.clone());
                1 + left_depth.max(right_depth)
            }
        }
    }

    // BFS
    pub fn max_depth_v2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        let mut depth = 0;

        while !queue.is_empty() {
            let level_size = queue.len();

            for _ in 0..level_size {
                let node = queue.pop_front().unwrap();
                let node_ref = node.borrow();

                if let Some(left) = &node_ref.left {
                    queue.push_back(left.clone());
                }
                if let Some(right) = &node_ref.right {
                    queue.push_back(right.clone());
                }
            }

            depth += 1;
        }

        depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let tree1 = TreeNode::from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_depth_v2(tree1), 3);

        let tree2 = TreeNode::from_vec(vec![Some(1), None, Some(2)]);
        assert_eq!(Solution::max_depth_v2(tree2), 2);

        let tree3 = TreeNode::from_vec(vec![]);
        assert_eq!(Solution::max_depth_v2(tree3), 0);
    }
}
