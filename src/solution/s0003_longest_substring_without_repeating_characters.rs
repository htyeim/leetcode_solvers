/**
[3] Longest Substring Without Repeating Characters

Given a string `s`, find the length of the **longest** **substring** without repeating characters.

**Example 1:**

```
Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.

```

**Example 2:**

```
Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.

```

**Example 3:**

```
Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

```

**Constraints:**

* `0 <= s.length <= 5 * 10<sup>4</sup>`
* `s` consists of English letters, digits, symbols and spaces.
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    ///doc
    #[allow(unused)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_indexes = std::collections::HashMap::with_capacity(s.len());
        let mut start_index = 0;
        let mut max_len = 0;
        for (index, c) in s.chars().enumerate() {
            match last_indexes.get(&c) {
                Some(last_index) => {
                    start_index = *last_index + 1;
                    last_indexes.insert(c, index);
                }
                None => {
                    last_indexes.insert(c, index);
                }
            };
            let this_len = index - start_index;
            if this_len > max_len {
                max_len = this_len;
            }
        }
        let ret = max_len as i32 + 1;
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
    fn test_3() {
        logger::init_logger();
        info!("test start ...");

        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );

        info!("test end===============");
    }
}
