//! # 通用工具和数据结构
//!
//! 这个模块包含了 LeetCode 中常用的数据结构定义和辅助函数。
//! 随着解题数量的增加，这里会逐渐添加更多常用类型。

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// LeetCode 中常用的链表节点定义
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    /// 从向量创建链表（LeetCode 常用格式）
    #[allow(dead_code)]
    pub fn from_vec(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vals.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    /// 将链表转换为向量（LeetCode 常用格式）
    #[allow(dead_code)]
    pub fn to_vec(&self) -> Vec<i32> {
        let mut result = vec![self.val];
        let mut current = &self.next;
        while let Some(node) = current {
            result.push(node.val);
            current = &node.next;
        }
        result
    }
}

/// LeetCode 中常用的二叉树节点定义
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    /// 从 LeetCode 格式的向量创建二叉树
    ///
    /// # 参数
    /// - `vec`: 向量中的元素为 `Some(i32)` 表示节点值，`None` 表示空节点
    ///
    /// # 示例
    /// ```
    /// use leetcode::utils::TreeNode;
    ///
    /// // 创建二叉树: [1, 2, 3]
    /// //     1
    /// //    / \
    /// //   2   3
    /// let tree = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
    /// ```
    #[allow(dead_code)]
    pub fn from_vec(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vec.is_empty() || vec[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut index = 1;

        while let Some(node) = queue.pop_front() {
            // 处理左子节点
            if index < vec.len() {
                if let Some(val) = vec[index] {
                    let left_child = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left_child));
                    queue.push_back(left_child);
                }
                index += 1;
            }

            // 处理右子节点
            if index < vec.len() {
                if let Some(val) = vec[index] {
                    let right_child = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(Rc::clone(&right_child));
                    queue.push_back(right_child);
                }
                index += 1;
            }
        }

        Some(root)
    }

    /// 将二叉树转换为 LeetCode 格式的向量（层序遍历）
    ///
    /// # 返回
    /// 返回一个向量，其中 `Some(i32)` 表示节点值，`None` 表示空节点
    ///
    /// # 示例
    /// ```
    /// use leetcode::utils::TreeNode;
    ///
    /// let tree = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
    /// let vec = TreeNode::to_vec(&tree);
    /// assert_eq!(vec, vec![Some(1), Some(2), Some(3)]);
    /// ```
    #[allow(dead_code)]
    pub fn to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut result = Vec::new();

        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        while let Some(node) = queue.pop_front() {
            if let Some(n) = node {
                let n_borrow = n.borrow();
                result.push(Some(n_borrow.val));

                queue.push_back(n_borrow.left.clone());
                queue.push_back(n_borrow.right.clone());
            } else {
                result.push(None);
            }
        }

        // 移除末尾连续的 None
        while let Some(None) = result.last() {
            result.pop();
        }

        result
    }
}

// 注意：随着解题的进行，你可能会遇到其他常用数据结构
// 例如：图节点（Node）、坐标点（Point）等
// 可以按需添加到这里

/// 图节点（多叉树节点）的定义（示例）
/// 用于 LeetCode 中需要处理图或 N 叉树的题目
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GraphNode {
    pub val: i32,
    pub neighbors: Vec<Option<Rc<RefCell<GraphNode>>>>,
}

impl GraphNode {
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        GraphNode {
            val,
            neighbors: Vec::new(),
        }
    }
}

/// 二维坐标点（示例）
/// 用于处理矩阵、网格相关的题目
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[allow(dead_code)]
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
