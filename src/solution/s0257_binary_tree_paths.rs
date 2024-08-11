/**
 * [257] Binary Tree Paths
 *
 * Given the root of a binary tree, return all root-to-leaf paths in any order.
 * A leaf is a node with no children.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/12/paths-tree.jpg" style="width: 207px; height: 293px;" />
 * Input: root = [1,2,3,null,5]
 * Output: ["1->2->5","1->3"]
 *
 * <strong class="example">Example 2:
 *
 * Input: root = [1]
 * Output: ["1"]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 100].
 * 	-100 <= Node.val <= 100
 *
 */
use log::debug;
pub struct Solution {}
use crate::util::tree::TreeNode;

// problem: https://leetcode.com/problems/binary-tree-paths/
// discuss: https://leetcode.com/problems/binary-tree-paths/discuss/?currentPage=1&orderBy=most_votes&query=

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
    #[allow(unused)]
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut res = Vec::new();
        Solution::helper(root, "".to_owned(), &mut res);
        debug!("{:?}", res);
        res
    }
    fn helper(root: Option<Rc<RefCell<TreeNode>>>, path: String, res: &mut Vec<String>) {
        if let Some(inner) = root {
            if inner.borrow().left.is_none() && inner.borrow().right.is_none() {
                res.push(format!("{}{}", path, inner.borrow().val));
            } else {
                let path = format!("{}{}->", path, inner.borrow().val);
                Solution::helper(inner.borrow().left.clone(), path.clone(), res);
                Solution::helper(inner.borrow().right.clone(), path, res);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::logger;
    use log::info;

    use crate::util::tree::to_tree;
    #[test]
    fn test_257() {
        logger::init_logger();
        info!("test start ...");

        assert_eq!(
            Solution::binary_tree_paths(tree![1, 2, 3, null, 5]),
            vec_string!["1->2->5", "1->3"]
        );

        info!("test end===============");
    }
}
