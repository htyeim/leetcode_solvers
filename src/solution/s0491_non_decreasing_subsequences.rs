/**
[491] Non-decreasing Subsequences

Given an integer array `nums`, return *all the different possible non-decreasing subsequences of the given array with at least two elements*. You may return the answer in **any order**.

**Example 1:**

```sh
Input: nums = [4,6,7,7]
Output: [[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]

```

**Example 2:**

```sh
Input: nums = [4,4,3,2,1]
Output: [[4,4]]

```

**Constraints:**

* `1 <= nums.length <= 15`
* `-100 <= nums[i] <= 100`
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/non-decreasing-subsequences/
// discuss: https://leetcode.com/problems/non-decreasing-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashSet;
impl Solution {
    ///doc
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::backtracking(&nums[..], &mut vec![], &mut res);
        res.sort();
        res
    }
    fn backtracking(nums: &[i32], path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if path.len() > 1 {
            res.push(path.clone());
        }
        let cur_back = match path.last() {
            None => i32::MIN,
            Some(num) => *num,
        };
        let mut used: HashSet<i32> = HashSet::new();
        for i in 0..nums.len() {
            let cur_value = nums[i];
            if cur_back > cur_value {
                continue;
            }
            if used.contains(&cur_value) {
                continue;
            }
            used.insert(cur_value);
            path.push(cur_value);
            Self::backtracking(&nums[(i + 1)..], path, res);
            path.pop();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::logger;
    use log::info;
    #[test]
    fn test_491() {
        logger::init_logger();
        info!("test start ...");

        {
            let mut res = vec![vec![1, 6], vec![1, 1], vec![1, 1, 1]];
            res.sort();
            assert_eq!(Solution::find_subsequences(vec![1, 6, 1, 1]), res);
        }
        {
            let mut res = vec![
                vec![4, 6],
                vec![4, 7],
                vec![4, 6, 7],
                vec![4, 6, 7, 7],
                vec![6, 7],
                vec![6, 7, 7],
                vec![7, 7],
                vec![4, 7, 7],
            ];
            res.sort();
            assert_eq!(Solution::find_subsequences(vec![4, 6, 7, 7]), res);
        }
        info!("test end===============");
    }
}
