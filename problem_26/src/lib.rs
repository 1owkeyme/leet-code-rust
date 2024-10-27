#[cfg(test)]
mod tests {
    use solution::Solution;

    trait Remove {
        fn remove_duplicates(nums: &mut Vec<i32>) -> i32;
    }

    impl Remove for Solution {
        fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            let mut li = 0;
            for ri in 1..nums.len() {
                if nums[li] != nums[ri] {
                    li += 1;
                    if li != ri {
                        nums[li] = nums[ri];
                    }
                }
            }
            (li + 1) as i32
        }
    }

    #[test]
    fn test_1() {
        let mut nums = vec![1, 1, 2];
        let ans = 2;
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, ans);
        assert_eq!(nums[..(ans as usize)], [1, 2]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let ans = 5;
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, ans);
        assert_eq!(nums[..(ans as usize)], [0, 1, 2, 3, 4]);
    }
}
