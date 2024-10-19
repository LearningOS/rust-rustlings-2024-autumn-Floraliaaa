/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DON
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
        insert_node(&mut self.root, value);
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        search_node(&self.root, value)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => insert_node(&mut self.left, value),
            Ordering::Greater => insert_node(&mut self.right, value),
            Ordering::Equal => return,
        }
    }
}

fn insert_node<T>(node: &mut Option<Box<TreeNode<T>>>, value: T)
where
    T: Ord,
{
    match node {
        Some(node) => match value.cmp(&node.value) {
            Ordering::Less => insert_node(&mut node.left, value),
            Ordering::Greater => insert_node(&mut node.right, value),
            Ordering::Equal => return,
        },
        None => *node = Some(Box::new(TreeNode::new(value))),
    }
}

fn search_node<T>(node: &Option<Box<TreeNode<T>>>, value: T) -> bool
where
    T: Ord,
{
    match node {
        Some(node) => match value.cmp(&node.value) {
            Ordering::Less => search_node(&node.left, value),
            Ordering::Greater => search_node(&node.right, value),
            Ordering::Equal => true,
        },
        None => false
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


