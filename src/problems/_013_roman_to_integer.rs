#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn char_to_value(c:char) -> i32 {
            match c {
                'I' => 1,
                'V'=> 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }

        let chars: Vec<char> = s.chars().collect();
        let mut total = 0;
        for i in 0..chars.len() {
            let current_value = char_to_value(chars[i]);
            if i + 1 < chars.len() {
                let next_value = char_to_value(chars[i+1]);
                if current_value < next_value {
                    total -= current_value;
                } else {
                    total += current_value;
                }
            } else {
                total += current_value;
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}