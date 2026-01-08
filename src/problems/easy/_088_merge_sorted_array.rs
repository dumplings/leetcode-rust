#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = m + n - 1;

        while i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }

        while j >= 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_merge() {
        let mut arr1_1 = vec![1, 2, 3, 0, 0, 0];
        let mut arr1_2 = vec![2, 5, 6];
        Solution::merge(&mut arr1_1, 3, &mut arr1_2, 3);
        assert_eq!(arr1_1, vec![1, 2, 2, 3, 5, 6]);

        // let mut arr2_1 = vec![1];
        // let mut arr2_2: Vec<i32> = vec![];
        // Solution::merge(&mut arr2_1, 1, &mut arr2_2, 0);
        // assert_eq!(arr2_1, vec![1]);
        //
        // let mut arr3_1 = vec![0];
        // let mut arr3_2: Vec<i32> = vec![1];
        // Solution::merge(&mut arr3_1, 0, &mut arr3_2, 1);
        // assert_eq!(arr2_1, vec![1]);
    }
}
