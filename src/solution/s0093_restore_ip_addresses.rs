use crate::util::point;

/**
[93] Restore IP Addresses

A **valid IP address** consists of exactly four integers separated by single dots. Each integer is between `0` and `255` (**inclusive**) and cannot have leading zeros.

* For example, `"0.1.2.201"` and `"192.168.1.1"` are **valid** IP addresses, but `"0.011.255.245"`, `"192.168.1.312"` and `"192.168@1.1"` are **invalid** IP addresses.

Given a string `s` containing only digits, return *all possible valid IP addresses that can be formed by inserting dots into* `s`. You are **not** allowed to reorder or remove any digits in `s`. You may return the valid IP addresses in **any** order.

**Example 1:**

```sh
Input: s = "25525511135"
Output: ["255.255.11.135","255.255.111.35"]

```

**Example 2:**

```sh
Input: s = "0000"
Output: ["0.0.0.0"]

```

**Example 3:**

```sh
Input: s = "101023"
Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]

```

**Constraints:**

* `1 <= s.length <= 20`
* `s` consists of digits only.
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/restore-ip-addresses/
// discuss: https://leetcode.com/problems/restore-ip-addresses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    ///doc
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let len = s.len();
        if len < 4 || len > 12 {
            return result;
        }
        Self::backtracking(&s, 0, 1, len, &mut vec![], &mut result);
        result
    }

    fn backtracking(
        s: &String,
        start_index: usize,
        idx: usize,
        len: usize,
        path: &mut Vec<String>,
        res: &mut Vec<String>,
    ) {
        if idx == 5 {
            if start_index == len {
                res.push(path.join("."));
            }
            return;
        }
        let end_index = (start_index + 4).min(len);
        for cur_end in start_index..=end_index {
            let (b, sub) = Self::is_valid(&s[start_index..cur_end]);
            if b {
                path.push(sub);
                Self::backtracking(&s, cur_end, idx + 1, len, path, res);
                path.pop();
            }
        }
    }

    fn is_valid(segment: &str) -> (bool, String) {
        if segment.len() > 3 {
            return (false, segment.to_string());
        }
        if segment.starts_with('0') && segment.len() > 1 {
            return (false, segment.to_string());
        }
        if let Ok(num) = segment.parse::<i32>() {
            if num >= 0 && num <= 255 {
                return (true, segment.to_string());
            }
        }
        (false, segment.to_string())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::logger;
    use log::info;
    #[test]
    fn test_93() {
        logger::init_logger();
        info!("test start ...");

        assert_eq!(
            Solution::restore_ip_addresses("25525511135".to_string()),
            vec!["255.255.11.135", "255.255.111.35"]
        );
        assert_eq!(
            Solution::restore_ip_addresses("0000".to_string()),
            vec!["0.0.0.0"]
        );
        assert_eq!(
            Solution::restore_ip_addresses("101023".to_string()),
            vec![
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3"
            ]
        );
        info!("test end===============");
    }
}
