use crate::utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    /// 方法一：递归实现
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::inorder_recursive(root.as_ref(), &mut result);
        result
    }
    fn inorder_recursive(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            // 递归遍历左子树
            Self::inorder_recursive(n.borrow().left.as_ref(), result);
            // 访问当前节点
            result.push(n.borrow().val);
            // 递归遍历右子树
            Self::inorder_recursive(n.borrow().right.as_ref(), result);
        }
    }

    /// 方法二：迭代实现（使用栈）
    pub fn inorder_traversal_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = root;

        while current.is_some() || !stack.is_empty() {
            // 将所有左子节点入栈
            while let Some(node) = current {
                stack.push(Rc::clone(&node));
                current = node.borrow().left.clone();
            }

            // 弹出栈顶节点
            if let Some(node) = stack.pop() {
                // 访问节点值
                result.push(node.borrow().val);
                // 转向右子节点
                current = node.borrow().right.clone();
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let root = TreeNode::from_vec(vec![Some(1), None, Some(2), Some(3)]);
        let expected = vec![1, 3, 2];

        assert_eq!(Solution::inorder_traversal(root.clone()), expected);
        assert_eq!(
            Solution::inorder_traversal_iterative(root.clone()),
            expected
        );
    }
}
