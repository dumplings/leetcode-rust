#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_element() {
        assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2);
        assert_eq!(
            Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2),
            5
        );
    }
}
