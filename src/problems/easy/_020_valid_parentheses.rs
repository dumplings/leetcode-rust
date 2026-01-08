#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut arr = vec![];

        for c in s.chars() {
            match c {
                '(' | '{' | '[' => arr.push(c),
                ')' => {
                    if arr.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if arr.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if arr.pop() != Some('{') {
                        return false;
                    }
                }
                _ => {}
            }
        }
        arr.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("([])".to_string()), true);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
        assert_eq!(Solution::is_valid("]".to_string()), false);
        assert_eq!(Solution::is_valid("(".to_string()), false);
    }
}
