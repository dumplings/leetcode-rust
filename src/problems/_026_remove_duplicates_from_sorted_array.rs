#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut write_index = 1;

        for read_index in 1..nums.len() {
            if nums[read_index] != nums[read_index - 1] {
                nums[write_index] = nums[read_index];
                write_index += 1;
            }
        }

        nums.truncate(write_index);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_duplicates() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 2]), 2);
        assert_eq!(
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
            5
        );
    }
}
