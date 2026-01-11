use crate::utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    /// 比较两棵树是否相同的递归定义：
    // 1. 终止条件（Base Cases）：
    // - 如果两棵树的当前节点都为空（都是None）⇒ 返回 true
    // - 如果只有一个为空，另一个不为空 ⇒ 返回 false
    // - 如果两个节点值不相等 ⇒ 返回 false
    //
    // 2. 递归步骤（Recursive Step）：
    // - 当前节点值相等 ⇒ 继续递归比较左右子树
    // - 需要左子树相同 且 右子树相同
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // 使用元组模式匹配，同时检查两个Option
        match (p, q) {
            // 两个都是None：空树相同
            (None, None) => true,
            // 两个都是Some：需要进一步比较
            (Some(p_node), Some(q_node)) => {
                // 使用borrow()获取内部不可变引用，不消耗所有权
                let p_ref = p_node.borrow();
                let q_ref = q_node.borrow();

                // 比较当前节点值，并递归比较左右子树
                // 使用&&运算符确保所有条件都满足
                p_ref.val == q_ref.val
                    && Self::is_same_tree(p_ref.left.clone(), q_ref.left.clone())
                    && Self::is_same_tree(p_ref.right.clone(), q_ref.right.clone())
            }
            // 一个None一个Some：结构不同
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        let q = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::is_same_tree(p, q), true);

        let p = TreeNode::from_vec(vec![Some(1), Some(2)]);
        let q = TreeNode::from_vec(vec![Some(1), Some(3)]);
        assert_eq!(Solution::is_same_tree(p, q), false);

        let p = TreeNode::from_vec(vec![Some(1), Some(2)]);
        let q = TreeNode::from_vec(vec![Some(1), None, Some(2)]);
        assert_eq!(Solution::is_same_tree(p, q), false);

        let p = TreeNode::from_vec(vec![Some(1)]);
        let q = TreeNode::from_vec(vec![]); // 空树
        assert_eq!(Solution::is_same_tree(p, q), false);
    }
}
