#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut left = 0;
        let x = x as i64;
        let mut right = x;
        while left <= right {
            let mid = left + (right - left) / 2;
            let square = mid * mid;
            if square == x {
                return mid as i32;
            } else if square < x as i64 {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        right as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_my_sqrt() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
