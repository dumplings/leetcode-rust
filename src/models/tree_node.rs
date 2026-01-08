use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// 定义二叉树节点（LeetCode标准定义，无需改动）
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// ====== 1. 将 Vec<Option<i32>> 转成二叉树 ======
#[allow(dead_code)]
pub fn vec_to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    // 🚨 重要：如果输入为空或根节点为null，直接返回空树
    if vec.is_empty() || vec[0].is_none() {
        return None;
    }

    // 🌳 步骤1：创建根节点（第一个元素）
    let root = Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap())));
    // 📌 解释：Rc<RefCell> = 允许多个节点共享这个树节点的"借阅证"
    //          unwrap()：安全地从Some(1)取出1，如果为None会panic（但前面已检查）

    // 🧭 步骤2：创建队列（双端队列）用于层序遍历
    let mut queue = VecDeque::new();
    queue.push_back(root.clone()); // 📌 从队尾加入根节点（准备遍历）

    // 📋 步骤3：从第二个元素开始遍历（索引1开始）
    let mut index = 1;
    while index < vec.len() {
        // 🔄 步骤3.1：从队头取出当前节点（要处理的节点）
        if let Some(node) = queue.pop_front() {
            // 🌲 步骤3.2：处理左子节点（当前元素的下一个位置）
            if index < vec.len() && vec[index].is_some() {
                // ✅ 创建左子节点
                let left = Rc::new(RefCell::new(TreeNode::new(vec[index].unwrap())));
                // 🔧 设置当前节点的左子树（需要借用可变）
                node.borrow_mut().left = Some(left.clone());
                // 🔄 将左子节点加入队列（准备后续处理）
                queue.push_back(left);
            }
            index += 1; // 📌 移动到下一个元素

            // 🌲 步骤3.3：处理右子节点（再下一个位置）
            if index < vec.len() && vec[index].is_some() {
                // ✅ 创建右子节点
                let right = Rc::new(RefCell::new(TreeNode::new(vec[index].unwrap())));
                // 🔧 设置当前节点的右子树
                node.borrow_mut().right = Some(right.clone());
                // 🔄 将右子节点加入队列
                queue.push_back(right);
            }
            index += 1; // 📌 移动到下一个元素
        }
    }

    // 🎯 最终返回根节点（整个树）
    Some(root)
}

// ====== 2. 将二叉树转成 Vec<Option<i32>> ======
#[allow(dead_code)]
pub fn tree_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    // 🚨 重要：如果树为空，返回空向量
    if root.is_none() {
        return Vec::new();
    }

    // 📌 创建结果向量（将存储层序遍历结果）
    let mut result = Vec::new();
    // 🧭 创建队列用于遍历
    let mut queue = VecDeque::new();
    // 📌 将根节点加入队列（准备处理）
    queue.push_back(root);

    // 🔄 层序遍历（BFS）的核心循环
    while let Some(node) = queue.pop_front() {
        // 🌳 处理当前节点（可能是Some或None）
        if let Some(node) = node {
            // ✅ 1. 将当前节点值加入结果（Some(val)）
            result.push(Some(node.borrow().val));

            // ✅ 2. 将左子节点加入队列（如果存在）
            queue.push_back(node.borrow().left.clone());

            // ✅ 3. 将右子节点加入队列（如果存在）
            queue.push_back(node.borrow().right.clone());
        } else {
            // ❌ 4. 如果节点为空（None），加入None到结果
            result.push(None);
        }
    }

    // 🎯 返回层序遍历结果
    result
}
