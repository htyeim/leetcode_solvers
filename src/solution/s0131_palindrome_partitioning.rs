/**
[131] Palindrome Partitioning

Given a string `s`, partition `s` such that every substring of the partition is a **palindrome**. Return *all possible palindrome partitioning of* `s`.

**Example 1:**

```sh
Input: s = "aab"
Output: [["a","a","b"],["aa","b"]]

```

**Example 2:**

```sh
Input: s = "a"
Output: [["a"]]

```

**Constraints:**

* `1 <= s.length <= 16`
* `s` contains only lowercase English letters.
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-partitioning/
// discuss: https://leetcode.com/problems/palindrome-partitioning/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn is_palindrome(s: &str) -> bool {
        let cs: Vec<char> = s.chars().collect();
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            if cs[left] != cs[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }

    fn backtrack(
        s: &String,
        start: usize,
        current_partition: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        if start == s.len() {
            result.push(current_partition.clone());
            return;
        }

        for i in start..s.len() {
            if Self::is_palindrome(&s[start..=i]) {
                current_partition.push(s[start..=i].to_string());
                Self::backtrack(&s, i + 1, current_partition, result);
                current_partition.pop();
            }
        }
    }

    ///doc
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut current_partition: Vec<String> = Vec::new();
        Self::backtrack(&s, 0, &mut current_partition, &mut result);
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
    fn test_131() {
        logger::init_logger();
        info!("test start ...");

        assert_eq!(
            Solution::partition("aab".to_owned()),
            vec![vec_string!["a", "a", "b"], vec_string!["aa", "b"],]
        );
        assert_eq!(
            Solution::partition("aaa".to_owned()),
            vec![
                vec_string!["a", "a", "a"],
                vec_string!["a", "aa"],
                vec_string!["aa", "a"],
                vec_string!["aaa"],
            ]
        );
        info!("test end===============");
    }
}
