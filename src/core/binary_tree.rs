use std::borrow::BorrowMut;


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    #[allow(dead_code)]
    pub fn reverse_tree(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        let root = root;
        if root.is_none() {
            return None;
        }
        let mut root = root.unwrap();
        let left = root.left;
        root.left = root.right;
        root.right = left;
        root.left = Self::reverse_tree(root.left);
        root.right = Self::reverse_tree(root.right);
        Some(root)
    }
    #[allow(dead_code)]
    #[doc = "Reverse a binary tree using recursion and match approach"]
    pub fn reverse_tree_rc(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
      match root {
        None => None,
        Some(mut node) => {

          let borrow_node: &mut TreeNode = node.borrow_mut();
          let left = borrow_node.borrow_mut().left.take();
          let right = borrow_node.borrow_mut().right.take();

          borrow_node.left = Self::reverse_tree_rc(left);
          borrow_node.right = Self::reverse_tree_rc(right);

          Some(node)
        }
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_tree() {
        let mut root = TreeNode::new(4);
        let mut left = TreeNode::new(2);
        let mut right = TreeNode::new(7);
        let left_left = TreeNode::new(1);
        let left_right = TreeNode::new(3);
        let right_left = TreeNode::new(6);
        let right_right = TreeNode::new(9);
        left.left = Some(Box::new(left_left));
        left.right = Some(Box::new(left_right));
        right.left = Some(Box::new(right_left));
        right.right = Some(Box::new(right_right));
        root.left = Some(Box::new(left));
        root.right = Some(Box::new(right));
        let result = TreeNode::reverse_tree(Some(Box::new(root)));
        let unwrapped_result = result.unwrap();
        assert_eq!(unwrapped_result.val, 4);
        assert_eq!(unwrapped_result.left.unwrap().val, 7);
        assert_eq!(unwrapped_result.right.unwrap().val, 2);
    }

    #[test]
    fn test_reverse_tree_rc() {
        let mut root = TreeNode::new(4);
        let mut left = TreeNode::new(2);
        let mut right = TreeNode::new(7);
        let left_left = TreeNode::new(1);
        let left_right = TreeNode::new(3);
        let right_left = TreeNode::new(6);
        let right_right = TreeNode::new(9);
        left.left = Some(Box::new(left_left));
        left.right = Some(Box::new(left_right));
        right.left = Some(Box::new(right_left));
        right.right = Some(Box::new(right_right));
        root.left = Some(Box::new(left));
        root.right = Some(Box::new(right));
        let result = TreeNode::reverse_tree_rc(Some(Box::new(root)));
        let unwrapped_result = result.unwrap();
        assert_eq!(unwrapped_result.val, 4);
        assert_eq!(unwrapped_result.left.unwrap().val, 2);
        assert_eq!(unwrapped_result.right.unwrap().val, 7);
    }
}