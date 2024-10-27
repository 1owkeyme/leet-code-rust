#[cfg(test)]
mod tests {
    use solution::Solution;

    trait Remove {
        fn remove_duplicates(nums: &mut Vec<i32>) -> i32;
    }

    impl Remove for Solution {
        fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            if nums.len() <= 2 {
                return nums.len() as i32;
            }
            let mut li = 2;
            for ri in 2..nums.len() {
                if nums[ri] == nums[li - 2] {
                    continue;
                }
                nums[li] = nums[ri];
                li += 1;
            }
            li as i32
        }
    }

    #[test]
    fn test_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let ans = 5;
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, ans);
        assert_eq!(nums[..(ans as usize)], [1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let ans = 7;
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, ans);
        assert_eq!(nums[..(ans as usize)], [0, 0, 1, 1, 2, 3, 3]);
    }
}
