//! 自动创建 LeetCode 题目的辅助脚本
//!
//! 使用方法：
//! cargo run --bin new_problem <题号> "<题目名称>" <难度>
//!
//! 示例：
//! cargo run --bin new_problem 123 "Best Time to Buy and Sell Stock" easy

use std::fs;
use std::path::Path;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // 检查参数数量
    if args.len() != 4 {
        print_usage();
        process::exit(1);
    }

    let problem_num = &args[1];
    let problem_name = &args[2];
    let difficulty = args[3].to_lowercase();

    // 验证难度参数
    if !["easy", "medium", "hard"].contains(&difficulty.as_str()) {
        eprintln!("错误：难度必须是 'easy'，'medium' 或 'hard'");
        print_usage();
        process::exit(1);
    }

    // 验证题号
    let problem_num: u32 = match problem_num.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("错误：题号必须是正整数");
            process::exit(1);
        }
    };

    // 创建新题目
    match create_problem(problem_num, problem_name, &difficulty) {
        Ok(file_path) => {
            println!("✅ 成功创建题目文件: {:?}", file_path);
            println!("📝 请编辑文件并实现解题方法");
        }
        Err(e) => {
            eprintln!("❌ 创建题目失败: {}", e);
            process::exit(1);
        }
    }
}

fn create_problem(
    problem_num: u32,
    problem_name: &str,
    difficulty: &str,
) -> Result<String, String> {
    // 生成文件名
    let file_name = format!(
        "_{:0>3}_{}.rs",
        problem_num,
        sanitize_filename(problem_name)
    );

    // 生成文件路径
    let dir_path = format!("src/problems/{}/", difficulty);
    let file_path = format!("{}{}", dir_path, file_name);

    // 检查文件是否已存在
    if Path::new(&file_path).exists() {
        return Err(format!("文件已存在: {}", file_path));
    }

    // 生成文件内容
    let content = generate_template(problem_num, problem_name, difficulty);

    // 确保目录存在
    fs::create_dir_all(&dir_path).map_err(|e| format!("创建目录失败: {}", e))?;

    // 写入文件
    fs::write(&file_path, &content).map_err(|e| format!("写入文件失败: {}", e))?;

    // 更新 mod.rs 文件
    update_mod_file(&dir_path, &file_name).map_err(|e| format!("更新模块文件失败: {}", e))?;

    Ok(file_path)
}

fn sanitize_filename(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .replace("'", "")
        .replace("\"", "")
        .replace("?", "")
        .replace("!", "")
        .replace(",", "")
        .replace(",", "")
        .replace(".", "")
        .replace("(", "")
        .replace(")", "")
}

fn generate_template(problem_num: u32, problem_name: &str, difficulty: &str) -> String {
    let difficulty_capitalized = match difficulty {
        "easy" => "简单",
        "medium" => "中等",
        "hard" => "困难",
        _ => "未知",
    };
    format!(
        r#"//! # {}. {}
//!
//! **难度**: {}
//!
//! ## 题目描述
//!
//!
//! ## 示例
//! ```rust
//! // TODO: 添加示例
//! ```
//!
//! ## 解题思路
//!
//!
//! ## 复杂度分析
//! - 时间复杂度：
//! - 空间复杂度：
//!
//! ## 解法

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {{
    // TODO: 实现解题函数
    pub fn example() -> i32 {{
        0
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_example() {{
        assert_eq!(Solution::example(), 0);
    }}
}}
"#,
        problem_num, problem_name, difficulty_capitalized
    )
}

fn update_mod_file(dir_path: &str, file_name: &str) -> Result<(), String> {
    let mod_path = format!("{}mod.rs", dir_path);
    let module_name = file_name.trim_end_matches(".rs");
    let mod_line = format!("pub mod {};\n", module_name);

    // 读取现有的 mod.rs 内容
    let mut mod_content = match fs::read_to_string(&mod_path) {
        Ok(content) => content,
        Err(_) => {
            // 如果文件不存在，创建新文件
            format!(
                "//! {}难度题目\n\n",
                capitalize_first_letter(dir_path.split('/').nth_back(1).unwrap_or("unknown"))
            )
        }
    };

    // 检查是否已包含该模块
    if mod_content.contains(&mod_line) {
        return Ok(());
    }

    // 添加到文件末尾
    mod_content.push_str(&mod_line);

    // 按模块名排序
    let lines: Vec<&str> = mod_content.lines().collect();
    let mut pub_mod_lines: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("pub mod "))
        .copied()
        .collect();

    if pub_mod_lines.len() > 1 {
        pub_mod_lines.sort();
        let mut new_content = String::new();
        for line in lines {
            if !line.starts_with("pub mod ") && !line.trim().is_empty() {
                new_content.push_str(line);
                new_content.push('\n');
            }
        }

        // 添加排序后的模块声明
        if !new_content.trim().is_empty() {
            new_content.push('\n');
        }

        for line in pub_mod_lines {
            new_content.push_str(line);
            new_content.push('\n');
        }

        mod_content = new_content;
    }

    // 写回文件
    fs::write(&mod_path, mod_content).map_err(|e| format!("写入 mod.rs 失败: {}", e))?;

    Ok(())
}

fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn print_usage() {
    println!("LeetCode 题目创建工具");
    println!();
    println!("用法: cargo run --bin new_problem <题号> \"<题目名称>\" <难度>");
    println!();
    println!("参数说明:");
    println!("  题号: LeetCode 题目编号，如 1, 123, 987");
    println!("  题目名称: LeetCode 英文题目名称，如 \"Two Sum\"");
    println!("  难度: 题目难度，必须是 easy, medium 或 hard");
    println!();
    println!("示例:");
    println!("  cargo run --bin new_problem 1 \"Two Sum\" easy");
    println!("  cargo run --bin new_problem 123 \"Best Time to Buy and Sell Stock\" medium");
    println!(
        "  cargo run --bin new_problem 987 \"Vertical Order Traversal of a Binary Tree\" hard"
    );
}
