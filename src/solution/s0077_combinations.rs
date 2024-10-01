/**
[77] Combinations

Given two integers `n` and `k`, return *all possible combinations of* `k` *numbers chosen from the range* `[1, n]`.

You may return the answer in **any order**.

**Example 1:**

```sh
Input: n = 4, k = 2
Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
Explanation: There are 4 choose 2 = 6 total combinations.
Note that combinations are unordered, i.e., [1,2] and [2,1] are considered to be the same combination.

```

**Example 2:**

```sh
Input: n = 1, k = 1
Output: [[1]]
Explanation: There is 1 choose 1 = 1 total combination.

```

**Constraints:**

* `1 <= n <= 20`
* `1 <= k <= n`
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/combinations/
// discuss: https://leetcode.com/problems/combinations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn backtracing(start: i32, end: i32, k: i32, curr: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if k == 0 {
            ret.push(curr.clone());
            return;
        }
        if end - start + 1 < k {
            return;
        }
        curr.push(start);
        Self::backtracing(start + 1, end, k - 1, curr, ret);
        curr.pop();
        Self::backtracing(start + 1, end, k, curr, ret);
    }
    ///doc
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut curr: Vec<i32> = Vec::new();
        Self::backtracing(1, n, k, &mut curr, &mut ret);
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
    fn test_77() {
        logger::init_logger();
        info!("test start ...");

        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ]
        );
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combine(0, 1), empty);
        assert_eq!(Solution::combine(2, 1), vec![vec![1], vec![2]]);

        info!("test end===============");
    }
}
