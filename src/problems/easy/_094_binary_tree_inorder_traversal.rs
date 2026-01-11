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
        // 存储最终遍历结果的向量
        let mut result = Vec::new();
        // 使用栈来模拟递归调用栈，存储待处理的节点
        let mut stack = Vec::new();
        // current指针：当前正处理的节点，初始化为根节点
        let mut current = root;

        // 主循环条件：当前节点不为空或栈不为空
        // - current不为空：还有左子树需要处理
        // - 栈不为空：还有节点需要回溯处理
        while current.is_some() || !stack.is_empty() {
            // 第一阶段：尽可能深入左子树
            // 这个循环将当前节点及其所有左子节点压入栈中
            // 模拟递归中一直向左走到底的过程
            while let Some(node) = current {
                // 将当前节点压入栈中，以便后续回溯访问
                stack.push(Rc::clone(&node));

                // 将current移动到左子节点，继续深入左子树
                // 使用.clone()是因为Rc是引用计数智能指针，需要增加引用计数
                current = node.borrow().left.clone();
            }

            // 第二阶段：回溯并处理节点
            // 当不能再向左走时，从栈中弹出节点进行处理
            if let Some(node) = stack.pop() {
                // 访问当前节点值，这是『中序』的关键：
                // 在左子树处理完后、右子树处理前访问节点值
                result.push(node.borrow().val);

                // 转向处理右子树
                // 将current设置为当前节点的右子节点
                // 下一轮循环会先处理右子树的左子树（如果存在）
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
