/**
[701] Insert into a Binary Search Tree

You are given the `root` node of a binary search tree (BST) and a `value` to insert into the tree. Return *the root node of the BST after the insertion*. It is **guaranteed** that the new value does not exist in the original BST.

**Notice** that there may exist multiple valid ways for the insertion, as long as the tree remains a BST after insertion. You can return **any of them**.

**Example 1:**

![](https://assets.leetcode.com/uploads/2020/10/05/insertbst.jpg)

```sh
Input: root = [4,2,7,1,3], val = 5
Output: [4,2,7,1,3,5]
Explanation: Another accepted tree is:

```

**Example 2:**

```sh
Input: root = [40,20,60,10,30,50,70], val = 25
Output: [40,20,60,10,30,50,70,null,null,25]

```

**Example 3:**

```sh
Input: root = [4,2,7,1,3,null,null,null,null,null,null], val = 5
Output: [4,2,7,1,3,5]

```

**Constraints:**

* The number of nodes in the tree will be in the range `[0, 10<sup>4</sup>]`.
* `-10<sup>8</sup> <= Node.val <= 10<sup>8</sup>`
* All the values `Node.val` are **unique**.
* `-10<sup>8</sup> <= val <= 10<sup>8</sup>`
* It's **guaranteed** that `val` does not exist in the original BST.
*/

pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/insert-into-a-binary-search-tree/
// discuss: https://leetcode.com/problems/insert-into-a-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    ///doc
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            Some(node) => {
                let mut node_borrow = node.borrow_mut();
                if val < node_borrow.val {
                    node_borrow.left = Self::insert_into_bst(node_borrow.left.take(), val);
                } else {
                    node_borrow.right = Self::insert_into_bst(node_borrow.right.take(), val);
                }
                Some(node.clone())
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::logger;
    use crate::util::tree::to_tree;
    use log::info;
    #[test]
    fn test_701() {
        logger::init_logger();
        info!("test start ...");

        assert_eq!(
            Solution::insert_into_bst(tree![4, 2, 7, 1, 3], 5),
            tree![4, 2, 7, 1, 3, 5],
        );

        info!("test end===============");
    }
}
