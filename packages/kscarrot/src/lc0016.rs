use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        //隐藏条件最少3个
        let mut res = nums[0] + nums[1] + nums[2];
        let len = nums.len() - 2;
        for i in 0..len {
            //跳过相同值
            if i != 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut l, mut r) = (i + 1, len + 1);
            while l < r {
                let cur = nums[i] + nums[l] + nums[r];
                if (cur - target).abs() < (res - target).abs() {
                    res = cur;
                }

                match cur.cmp(&target) {
                    Ordering::Equal => return target,
                    Ordering::Less => l = l + 1,
                    Ordering::Greater => r = r - 1,
                }
            }
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lc0016() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 3], 1), 6);
        assert_eq!(
            Solution::three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82),
            82
        );
    }
}