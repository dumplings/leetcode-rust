# leetcode-rust

使用rust来完成leetcode算法练习

## 🛠️ 开发工具

项目包含一个辅助脚本，用于快速创建新的题目文件：

```bash
# 创建新题目
cargo run --bin new_problem <题号> "<题目名称>" <难度>

# 示例：
cargo run --bin new_problem 1 "Two Sum" easy
cargo run --bin new_problem 123 "Best Time to Buy and Sell Stock" medium
cargo run --bin new_problem 987 "Vertical Order Traversal of a Binary Tree" hard