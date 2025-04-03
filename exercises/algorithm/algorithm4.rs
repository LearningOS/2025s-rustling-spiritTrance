/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST  
    fn insert(&mut self, value: T) {  
        // 如果树为空，直接创建根节点  
        if self.root.is_none() {  
            self.root = Some(Box::new(TreeNode::new(value)));  
            return;  
        }  
        
        // 树不为空，需要递归查找插入位置  
        Self::insert_rec(&mut self.root, value);  
    }  

    // 递归辅助函数来插入节点  
    fn insert_rec(node: &mut Option<Box<TreeNode<T>>>, value: T) {  
        if let Some(node_ref) = node {  
            match value.cmp(&node_ref.value) {  
                Ordering::Less => {  
                    // 值小于当前节点，插入到左子树  
                    if node_ref.left.is_none() {  
                        node_ref.left = Some(Box::new(TreeNode::new(value)));  
                    } else {  
                        Self::insert_rec(&mut node_ref.left, value);  
                    }  
                },  
                Ordering::Greater => {  
                    // 值大于当前节点，插入到右子树  
                    if node_ref.right.is_none() {  
                        node_ref.right = Some(Box::new(TreeNode::new(value)));  
                    } else {  
                        Self::insert_rec(&mut node_ref.right, value);  
                    }  
                },  
                Ordering::Equal => {  
                    // 值等于当前节点，根据需求可以选择更新或忽略  
                    // 这里选择忽略重复值（BST通常不存储重复项）  
                    return;  
                }  
            }  
        }  
    }  

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        Self::search_recursive(&self.root, &value)
    }

    fn search_recursive(node: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
        if let Some(ptr) = node {
            if &ptr.value == value{
                return true;
            }
            return Self::search_recursive(&ptr.left, value) || Self::search_recursive(&ptr.right, value);
        }
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


