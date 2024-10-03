/**
[90] Subsets II

Given an integer array `nums` that may contain duplicates, return *all possible* *subsets* *(the power set)*.

The solution set **must not** contain duplicate subsets. Return the solution in **any order**.

**Example 1:**

```shInput: nums = [1,2,2]
Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]

```

**Example 2:**

```shInput: nums = [0]
Output: [[],[0]]

```

**Constraints:**

* `1 <= nums.length <= 10`
* `-10 <= nums[i] <= 10`
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/subsets-ii/
// discuss: https://leetcode.com/problems/subsets-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    ///doc
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut nums = nums;
        nums.sort();
        Self::backtracking(&nums, 0, nums.len(), &mut vec![], &mut res);
        res
    }
    fn backtracking(
        nums: &Vec<i32>,
        start_index: usize,
        vlen: usize,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        res.push(path.clone());
        if start_index == vlen {
            return;
        }
        for cur_index in start_index..vlen {
            if cur_index > start_index && nums[cur_index] == nums[cur_index - 1] {
                continue;
            }
            path.push(nums[cur_index]);
            Self::backtracking(nums, cur_index + 1, vlen, path, res);
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
    fn test_90() {
        logger::init_logger();
        info!("test start ...");
        {
            let mut res = vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![2, 2],
                vec![1, 2, 2],
            ];
            res.sort();
            assert_eq!(Solution::subsets_with_dup(vec![1, 2, 2]), res);
        }
        info!("test end===============");
    }
}
