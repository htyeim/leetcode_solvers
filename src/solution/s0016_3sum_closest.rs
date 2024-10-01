/**
[16] 3Sum Closest

Given an integer array `nums` of length `n` and an integer `target`, find three integers in `nums` such that the sum is closest to `target`.

Return *the sum of the three integers*.

You may assume that each input would have exactly one solution.

**Example 1:**

```sh
Input: nums = [-1,2,1,-4], target = 1
Output: 2
Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).

```

**Example 2:**

```sh
Input: nums = [0,0,0], target = 1
Output: 0
Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).

```

**Constraints:**

* `3 <= nums.length <= 500`
* `-1000 <= nums[i] <= 1000`
* `-10<sup>4</sup> <= target <= 10<sup>4</sup>`
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/3sum-closest/
// discuss: https://leetcode.com/problems/3sum-closest/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    ///doc
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        if n < 3 {
            return nums.iter().sum();
        }
        let mut nums = nums;
        nums.sort();
        let mut closest_sum = nums[0] + nums[1] + nums[2];
        for i in 0..n - 2 {
            let mut left = i + 1;
            let mut right = n - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum == target {
                    return sum;
                } else if sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
                if (sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = sum;
                }
            }
        }
        closest_sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::logger;
    use log::info;
    #[test]
    fn test_16() {
        logger::init_logger();
        info!("test start ...");

        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 3], 1), 6);
        assert_eq!(
            Solution::three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82),
            82
        );
        info!("test end===============");
    }
}
