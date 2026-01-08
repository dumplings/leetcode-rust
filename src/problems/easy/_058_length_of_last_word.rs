#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        // s.split_whitespace()
        //     .last()
        //     .map(|word| word.len() as i32)
        //     .unwrap_or(0)
        let bytes = s.as_bytes();
        let mut i = (bytes.len() - 1) as i32;

        while i >= 0 && bytes[i as usize] == b' ' {
            i -= 1;
        }

        let mut length = 0;
        while i >= 0 && bytes[i as usize] != b' ' {
            length += 1;
            i -= 1;
        }

        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }
}
