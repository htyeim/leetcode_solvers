/**
[216] Combination Sum III

Find all valid combinations of `k` numbers that sum up to `n` such that the following conditions are true:

* Only numbers `1` through `9` are used.
* Each number is used **at most once**.

Return *a list of all possible valid combinations*. The list must not contain the same combination twice, and the combinations may be returned in any order.

**Example 1:**

```sh
Input: k = 3, n = 7
Output: [[1,2,4]]
Explanation:
1 + 2 + 4 = 7
There are no other valid combinations.
```

**Example 2:**

```sh
Input: k = 3, n = 9
Output: [[1,2,6],[1,3,5],[2,3,4]]
Explanation:
1 + 2 + 6 = 9
1 + 3 + 5 = 9
2 + 3 + 4 = 9
There are no other valid combinations.

```

**Example 3:**

```sh
Input: k = 4, n = 1
Output: []
Explanation: There are no valid combinations.
Using 4 different numbers in the range [1,9], the smallest sum we can get is 1+2+3+4 = 10 and since 10 > 1, there are no valid combination.

```

**Constraints:**

* `2 <= k <= 9`
* `1 <= n <= 60`
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/combination-sum-iii/
// discuss: https://leetcode.com/problems/combination-sum-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn backtracing(
        start: i32,
        end: i32,
        k: i32,
        remain: i32,
        curr: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if remain == 0 {
            if k == 0 {
                ret.push(curr.clone());
            }
            return;
        }
        if remain < start || k < 1 || start > end {
            return;
        }

        curr.push(start);
        Self::backtracing(start + 1, end, k - 1, remain - start, curr, ret);
        curr.pop();
        Self::backtracing(start + 1, end, k, remain, curr, ret);
    }
    ///doc
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut curr: Vec<i32> = Vec::new();
        Self::backtracing(1, 9, k, n, &mut curr, &mut ret);
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
    fn test_216() {
        logger::init_logger();
        info!("test start ...");
        assert_eq!(
            Solution::combination_sum3(3, 9),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4],],
        );
        info!("test end===============");
    }
}
