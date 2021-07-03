use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut result = Vec::new();
        if len < 3 { return result; }
        let mut nums = nums;
        nums.sort_unstable();
        for a in 0..len {
            if nums[a] > 0 { break; }

            if a > 0 && nums[a] == nums[a - 1] {
                continue;
            }


            let mut b = a + 1;
            let mut c = len - 1;

            while b < c {
                match (nums[a] + nums[b] + nums[c]).cmp(&0) {
                    Ordering::Equal => {
                        result.push(vec![nums[a], nums[b], nums[c]]);
                        while b < c && nums[b] == nums[b + 1] {
                            b += 1
                        }
                        while b < c && nums[c] == nums[c - 1] {
                            c -= 1;
                        }
                        b += 1;
                        c -= 1;
                    }
                    Ordering::Less => b += 1,
                    Ordering::Greater => c -= 1,
                }
            }
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lc0015() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
}