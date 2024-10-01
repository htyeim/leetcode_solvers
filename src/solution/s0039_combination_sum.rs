/**
[39] Combination Sum

Given an array of **distinct** integers `candidates` and a target integer `target`, return *a list of all **unique combinations** of* `candidates` *where the chosen numbers sum to* `target`*.* You may return the combinations in **any order**.

The **same** number may be chosen from `candidates` an **unlimited number of times**. Two combinations are unique if the frequency of at least one of the chosen numbers is different.

The test cases are generated such that the number of unique combinations that sum up to `target` is less than `150` combinations for the given input.

**Example 1:**

```sh
Input: candidates = [2,3,6,7], target = 7
Output: [[2,2,3],[7]]
Explanation:
2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
7 is a candidate, and 7 = 7.
These are the only two combinations.

```

**Example 2:**

```sh
Input: candidates = [2,3,5], target = 8
Output: [[2,2,2,2],[2,3,3],[3,5]]

```

**Example 3:**

```sh
Input: candidates = [2], target = 1
Output: []

```

**Constraints:**

* `1 <= candidates.length <= 30`
* `2 <= candidates[i] <= 40`
* All elements of `candidates` are **distinct**.
* `1 <= target <= 40`
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/combination-sum/
// discuss: https://leetcode.com/problems/combination-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn backtrack(
        candidates: &[i32],
        start: usize,
        target: i32,
        current_combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(current_combination.clone());
            return;
        }

        for i in start..candidates.len() {
            if candidates[i] > target {
                break;
            }

            current_combination.push(candidates[i]);
            Self::backtrack(
                &candidates,
                i,
                target - candidates[i],
                current_combination,
                result,
            );
            current_combination.pop();
        }
    }

    ///doc
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut current_combination: Vec<i32> = Vec::new();

        let mut candidates = candidates.clone();
        candidates.sort();
        Self::backtrack(
            &candidates,
            0,
            target,
            &mut current_combination,
            &mut result,
        );
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::logger;
    use log::info;
    #[test]
    fn test_39() {
        logger::init_logger();
        info!("test start ...");

        assert_eq!(
            Solution::combination_sum(vec![1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3,], vec![7,],]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3,], vec![3, 5,],]
        );
        info!("test end===============");
    }
}
