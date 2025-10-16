use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let mut i = 1;
        let mut j = 2;

        for _ in 3..=n {
            let current = i + j;
            i = j;
            j = current;
        }
        j
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
