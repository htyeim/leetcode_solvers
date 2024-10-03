use std::collections::HashSet;

/**
[46] Permutations

Given an array `nums` of distinct integers, return all the possible permutations. You can return the answer in **any order**.

**Example 1:**

```shInput: nums = [1,2,3]
Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

```

**Example 2:**

```shInput: nums = [0,1]
Output: [[0,1],[1,0]]

```

**Example 3:**

```shInput: nums = [1]
Output: [[1]]

```

**Constraints:**

* `1 <= nums.length <= 6`
* `-10 <= nums[i] <= 10`
* All the integers of `nums` are **unique**.
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/permutations/
// discuss: https://leetcode.com/problems/permutations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    ///doc
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut used_index: HashSet<usize> = HashSet::new();
        let nlen = nums.len();
        Self::backtracking(&nums, nlen, &mut used_index, &mut vec![], &mut res);
        res
    }
    fn backtracking(
        nums: &Vec<i32>,
        nlen: usize,
        used_index: &mut HashSet<usize>,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if used_index.len() == nlen {
            res.push(path.clone());
            return;
        }
        for index in 0..nlen {
            if used_index.contains(&index) {
                continue;
            }
            used_index.insert(index);
            path.push(nums[index]);
            Self::backtracking(nums, nlen, used_index, path, res);
            path.pop();
            used_index.remove(&index);
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
    fn test_46() {
        logger::init_logger();
        info!("test start ...");
        {
            let mut res = vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ];
            res.sort();
            assert_eq!(Solution::permute(vec![1, 2, 3]), res);
        }
        info!("test end===============");
    }
}
