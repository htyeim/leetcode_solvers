use std::collections::{HashMap, HashSet};

/**
[40] Combination Sum II

Given a collection of candidate numbers (`candidates`) and a target number (`target`), find all unique combinations in `candidates` where the candidate numbers sum to `target`.

Each number in `candidates` may only be used **once** in the combination.

**Note:** The solution set must not contain duplicate combinations.

**Example 1:**

```sh
Input: candidates = [10,1,2,7,6,1,5], target = 8
Output:
[
[1,1,6],
[1,2,5],
[1,7],
[2,6]
]

```

**Example 2:**

```sh
Input: candidates = [2,5,2,1,2], target = 5
Output:
[
[1,2,2],
[5]
]

```

**Constraints:**

* `1 <= candidates.length <= 100`
* `1 <= candidates[i] <= 50`
* `1 <= target <= 30`
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/combination-sum-ii/
// discuss: https://leetcode.com/problems/combination-sum-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn backtrack(
        candidates: &Vec<i32>,
        start: usize,
        remain: i32,
        curr: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if remain < 0 {
            return;
        }
        if remain == 0 {
            ret.push(curr.clone());
            return;
        }
        let mut last_val: Option<i32> = None;
        for i in start..candidates.len() {
            let curr_val = *candidates.get(i).unwrap();
            if let Some(last_val) = last_val {
                if last_val == curr_val {
                    continue;
                }
            }
            last_val = Some(curr_val);
            curr.push(curr_val);
            Self::backtrack(candidates, i + 1, remain - curr_val, curr, ret);
            curr.pop();
        }
    }
    ///doc
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut curr: Vec<i32> = Vec::new();
        let mut candidates = candidates.clone();
        candidates.sort();
        Self::backtrack(&candidates, 0, target, &mut curr, &mut ret);
        return ret;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::logger;
    use log::info;
    #[test]
    fn test_40() {
        logger::init_logger();
        info!("test start ...");

        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6],]
        );

        assert_eq!(
            Solution::combination_sum2(vec![1, 1, 1, 1, 1, 1, 1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5,],]
        );
        info!("test end===============");
    }
}
