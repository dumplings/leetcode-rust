#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        // 最短字符串
        let min_len = strs.iter().map(|s| s.len()).min().unwrap();

        let first_str = &strs[0];

        for i in 0..min_len {
            let current_char = first_str.chars().nth(i).unwrap();

            for s in &strs[1..] {
                if s.chars().nth(i).unwrap() != current_char {
                    return first_str[..i].to_string();
                }
            }
        }
        first_str[..min_len].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "rececar".to_string(),
                "car".to_string(),
            ]),
            ""
        );
    }
}
