#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = digits;
        let mut carry = 1;

        for digit in result.iter_mut().rev() {
            let sum = *digit + carry;
            *digit = sum % 10;
            carry = sum / 10;
            if carry == 0 {
                break;
            }
        }

        if carry == 1 {
            result.insert(0, 1);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }
}
