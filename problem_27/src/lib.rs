#[cfg(test)]
mod tests {

    use solution::Solution;

    trait Remove {
        fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32;
    }

    impl Remove for Solution {
        fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
            let mut li = 0;
            for ri in 0..nums.len() {
                if nums[ri] != val {
                    if li != ri {
                        nums[li] = nums[ri];
                    }
                    li += 1;
                    continue;
                }

                if li == ri {
                    continue;
                }
            }
            li as i32
        }
    }

    #[test]
    fn test_1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let ans = 2;
        let k = Solution::remove_element(&mut nums, val);
        assert_eq!(k, ans);
        assert_eq!(nums[..(ans as usize)], [2, 2]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let ans = 5;
        let k = Solution::remove_element(&mut nums, val);
        assert_eq!(k, ans);
        assert_eq!(nums[..(ans as usize)], [0, 1, 3, 0, 4]);
    }
}
