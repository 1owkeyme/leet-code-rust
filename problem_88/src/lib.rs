#[cfg(test)]
mod tests {

    use solution::Solution;

    trait Merge {
        fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32);
    }

    impl Merge for Solution {
        fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            let mut mi = m - 1;
            let mut ni = n - 1;
            let mut r = m + n - 1;
            while ni >= 0 {
                if mi >= 0 && nums1[mi as usize] >= nums2[ni as usize] {
                    nums1[r as usize] = nums1[mi as usize];
                    mi -= 1;
                } else {
                    nums1[r as usize] = nums2[ni as usize];
                    ni -= 1;
                }
                r -= 1;
            }
        }
    }

    #[test]
    fn test_merge_example_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_merge_example_2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_merge_example_3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_merge_example_4() {
        let mut nums1 = vec![4, 0, 0, 0, 0, 0];
        let mut nums2 = vec![1, 2, 3, 5, 6];
        Solution::merge(&mut nums1, 1, &mut nums2, 5);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_empty() {
        let mut nums1 = vec![];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 0, &mut nums2, 0);
        assert_eq!(nums1, vec![]);
    }
}
