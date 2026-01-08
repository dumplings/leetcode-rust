// 定义链表节点（LeetCode标准链表定义）
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,                    // 节点存储的整数值
    pub next: Option<Box<ListNode>>, // 下一个节点（Box用于堆内存分配）
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None, // 新节点默认没有下一个节点
        }
    }
}

// ====== 1. 将 Vec<i32> 转成链表 ======
#[allow(dead_code)]
pub fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    // 🚨 重要：如果输入为空，直接返回空链表
    if vec.is_empty() {
        return None;
    }

    // 🌳 步骤1：创建一个虚拟头节点（用于构建链表）
    let mut head = None;

    // 🧭 步骤2：从向量末尾开始遍历（为什么？因为链表是头->尾顺序）
    // 为什么用rev()？因为Rust的链表是"头节点指向第一个元素"，而向量是[0,1,2]对应链表1->2->3
    // 如果从头遍历：[1,2,3] → 1→2→3，但这样需要不断修改头节点
    // 从尾遍历：[3,2,1] → 先创建3，再创建2→3，最后1→2→3（更高效）
    for &val in vec.iter().rev() {
        // ✅ 步骤2.1：创建新节点（值=val）
        let mut node = ListNode::new(val);

        // 🔧 步骤2.2：将新节点的next指向当前头节点（实现"插入到头部"）
        node.next = head;

        // 🔄 步骤2.3：更新头节点（新节点成为新的头）
        head = Some(Box::new(node));
    }

    // 🎯 最终返回头节点（整个链表）
    head
}

// ====== 2. 将链表转成 Vec<i32> ======
#[allow(dead_code)]
pub fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    // 🚨 重要：如果链表为空，返回空向量
    if head.is_none() {
        return Vec::new();
    }

    // 📌 创建结果向量（用于存储遍历值）
    let mut vec = Vec::new();

    // 🧭 创建当前节点指针（初始为头节点）
    let mut current = head;

    // 🔄 遍历链表的核心循环（直到链表结束）
    while let Some(node) = current {
        // ✅ 1. 将当前节点值加入结果向量
        vec.push(node.val);

        // ✅ 2. 移动到下一个节点（更新current）
        current = node.next;
    }

    // 🎯 返回结果向量
    vec
}
