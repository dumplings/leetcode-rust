#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // match haystack.find(&needle) {
        //     Some(i) => i as i32,
        //     None => -1,
        // }
        let n_len = needle.len();
        let h_len = haystack.len();

        if n_len > h_len {
            return -1;
        }
        for i in 0..(h_len - n_len + 1) {
            if &haystack[i..i + n_len] == needle {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_str_str() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
    }
}
