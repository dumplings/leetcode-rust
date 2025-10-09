#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let mut num = x;
        let mut reversed = 0;

        while num > reversed {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }

        num == reversed || num == reversed / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_palindrome_number() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
