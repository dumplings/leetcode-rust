use crate::utils::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    /// 递归
    // 1. 定义辅助函数is_mirror，比较两棵树是否互为镜像
    // 2. 轴对称 ⇔ 左子树和右子树互为镜像
    // 3. 两棵树互为镜像的条件：
    // - 两个根节点值相等
    // - 树A的左子树与树B的右子树互为镜像
    // - 树A的右子树与树B的左子树互为镜像
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node_ref = node.borrow();
                // 比较左右子树是否互为镜像
                Self::is_mirror(node_ref.left.clone(), node_ref.right.clone())
            }
        }
    }

    fn is_mirror(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            // 都不为空：比较值，并递归比较子树
            (Some(p_node), Some(q_node)) => {
                let p_ref = p_node.borrow();
                let q_ref = q_node.borrow();

                // 当前节点值必须相等，并且子树满足镜像关系
                p_ref.val == q_ref.val
                    && Self::is_mirror(p_ref.left.clone(), q_ref.right.clone())
                    && Self::is_mirror(p_ref.right.clone(), q_ref.left.clone())
            }
        }
    }

    /// 迭代队列实现 (BFS)
    // 1. 使用队列（或栈）存储需要比较的节点对
    // 2. 初始将根节点的左右子节点作为一对加入队列
    // 3. 每次从队列中取出一对节点进行比较：
    // - 都为空：继续下一对
    // - 一个空一个不空：返回false
    // - 值不相等：返回false
    // - 值相等：将外侧节点对和内侧节点对加入队列
    pub fn is_symmetric_v2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 1. 空树处理
        if root.is_none() {
            return true;
        }

        // 2. 使用双端队列存储需要比较的节点对
        let root_ref = root.as_ref().unwrap().borrow();
        let mut queue = VecDeque::new();

        // 3. 初始将根节点的左右子节点作为第一对比较对象
        // 因为轴对称性从根节点的左右子树开始
        queue.push_back((root_ref.left.clone(), root_ref.right.clone()));

        // 4. 主循环，直到队列为空
        // 队列中每个元素都是一对需要比较的节点
        while let Some((left, right)) = queue.pop_front() {
            // 5. 使用模式匹配处理所有可能的情况
            match (left, right) {
                (None, None) => continue,
                (None, Some(_)) | (Some(_), None) => return false,
                (Some(left_node), Some(right_node)) => {
                    let left_ref = left_node.borrow();
                    let right_ref = right_node.borrow();

                    if left_ref.val != right_ref.val {
                        return false;
                    }
                    // 将需要进一步比较的节点对加入队列
                    // 这里遵循轴对称的比较规则：
                    // 1. 左左 vs 右右（外侧对称）
                    // 2. 左右 vs 右左（内侧对称）
                    // 注意：使用push_back保证先入先出的BFS顺序
                    queue.push_back((left_ref.left.clone(), right_ref.right.clone()));
                    queue.push_back((left_ref.right.clone(), right_ref.left.clone()));
                }
            }
        }
        // 队列为空，没有发现不对称的情况
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let tree = TreeNode::from_vec(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ]);

        // 测试递归版本
        assert!(Solution::is_symmetric(tree.clone()));
        // 测试迭代版本
        assert!(Solution::is_symmetric_v2(tree.clone()));

        let tree = TreeNode::from_vec(vec![
            Some(1),
            Some(2),
            Some(2),
            None,
            Some(3),
            None,
            Some(3),
        ]);

        assert!(!Solution::is_symmetric(tree.clone()));
        assert!(!Solution::is_symmetric_v2(tree.clone()));
    }
}
