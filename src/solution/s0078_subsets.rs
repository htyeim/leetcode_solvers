/**
[78] Subsets

Given an integer array `nums` of **unique** elements, return *all possible* *subsets* *(the power set)*.

The solution set **must not** contain duplicate subsets. Return the solution in **any order**.

**Example 1:**

```sh
Input: nums = [1,2,3]
Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]

```

**Example 2:**

```sh
Input: nums = [0]
Output: [[],[0]]

```

**Constraints:**

* `1 <= nums.length <= 10`
* `-10 <= nums[i] <= 10`
* All the numbers of `nums` are **unique**.
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/subsets/
// discuss: https://leetcode.com/problems/subsets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    ///doc
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path: Vec<i32> = vec![];
        Self::backtracking(&nums, 0, nums.len(), &mut path, &mut res);
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
    fn test_78() {
        logger::init_logger();
        info!("test start ...");
        {
            let mut res = vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ];
            res.sort();
            assert_eq!(Solution::subsets(vec![1, 2, 3]), res);
        }
        {
            let mut res = vec![vec![], vec![0]];
            res.sort();
            assert_eq!(Solution::subsets(vec![0]), res);
        }
        info!("test end===============");
    }
}
