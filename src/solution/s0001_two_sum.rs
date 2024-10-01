/**
[1] Two Sum

Given an array of integers `nums` and an integer `target`, return *indices of the two numbers such that they add up to `target`*.

You may assume that each input would have ***exactly* one solution**, and you may not use the *same* element twice.

You can return the answer in any order.

**Example 1:**

```sh
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

```

**Example 2:**

```sh
Input: nums = [3,2,4], target = 6
Output: [1,2]

```

**Example 3:**

```sh
Input: nums = [3,3], target = 6
Output: [0,1]

```

**Constraints:**

* `2 <= nums.length <= 10<sup>4</sup>`
* `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`
* `-10<sup>9</sup> <= target <= 10<sup>9</sup>`
* **Only one valid answer exists.**

**Follow-up:** Can you come up with an algorithm that is less than `O(n<sup>2</sup>)` time complexity?
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/two-sum/
// discuss: https://leetcode.com/problems/two-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

impl Solution {
    ///doc
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (index, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                None => {
                    map.insert(num, index as i32);
                }
                Some(sub_index) => return vec![*sub_index, index as i32],
            }
        }
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::logger;
    use log::info;
    #[test]
    fn test_1() {
        logger::init_logger();
        info!("test start ...");
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));

        info!("test end===============");
    }
}
