/**
[17] Letter Combinations of a Phone Number

Given a string containing digits from `2-9` inclusive, return all possible letter combinations that the number could represent. Return the answer in **any order**.

A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

![](https://assets.leetcode.com/uploads/2022/03/15/1200px-telephone-keypad2svg.png)

**Example 1:**

```
Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]

```

**Example 2:**

```
Input: digits = ""
Output: []

```

**Example 3:**

```
Input: digits = "2"
Output: ["a","b","c"]

```

**Constraints:**

* `0 <= digits.length <= 4`
* `digits[i]` is a digit in the range `['2', '9']`.
*/

pub struct Solution {}

// problem: https://leetcode.com/problems/letter-combinations-of-a-phone-number/
// discuss: https://leetcode.com/problems/letter-combinations-of-a-phone-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn backtracing(
        digits: &String,
        start: usize,
        curr: &mut String,
        ret: &mut Vec<String>,
        digit_to_letters: &std::collections::HashMap<char, &'static str>,
    ) {
        if start == digits.len() {
            ret.push(curr.clone());
            return;
        }
        let c = digits.chars().nth(start).unwrap();
        for c in digit_to_letters.get(&c).unwrap().chars() {
            curr.push(c);
            Self::backtracing(digits, start + 1, curr, ret, digit_to_letters);
            curr.pop();
        }
    }
    ///doc
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        if digits.is_empty() {
            return ret;
        }
        let mut curr: String = String::new();

        let digit_to_letters: std::collections::HashMap<char, &'static str> =
            std::collections::HashMap::from([
                ('2', "abc"),
                ('3', "def"),
                ('4', "ghi"),
                ('5', "jkl"),
                ('6', "mno"),
                ('7', "pqrs"),
                ('8', "tuv"),
                ('9', "wxyz"),
            ]);

        Self::backtracing(&digits, 0, &mut curr, &mut ret, &digit_to_letters);
        ret.sort();
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
    fn test_17() {
        logger::init_logger();
        info!("test start ...");
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        info!("test end===============");
    }
}
