use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TreeNode<T> {
    pub data: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct BinarySearchTree<T> {
    pub root: Option<Box<TreeNode<T>>>,
}

#[allow(dead_code)]
impl<T: Ord + Display> TreeNode<T> {
    pub fn inorder_traverse(&self) {
        /*
        This wont work

        match &self.left {
            Some(node) => node.traverse(),
            None => return,
        }
        */

        if let Some(node) = &self.left {
            node.inorder_traverse()
        }

        println!("{}", self.data);

        if let Some(node) = &self.right {
            node.inorder_traverse()
        }
    }

    pub fn insert(&mut self, value: T) {
        if value < self.data {
            match &mut self.left {
                Some(node) => node.insert(value),
                None => {
                    self.left = Some(Box::new(TreeNode {
                        data: value,
                        left: None,
                        right: None,
                    }))
                }
            }
        } else if value > self.data {
            match &mut self.right {
                Some(node) => node.insert(value),
                None => {
                    self.right = Some(Box::new(TreeNode {
                        data: value,
                        left: None,
                        right: None,
                    }))
                }
            }
        }
    }

    pub fn search(&self, value: T) -> Option<&TreeNode<T>> {
        if value < self.data {
            match &self.left {
                Some(left) => left.search(value),
                None => None,
            }
        } else if value > self.data {
            match &self.right {
                Some(right) => right.search(value),
                None => None,
            }
        } else {
            return Some(self);
        }
    }
}

#[allow(dead_code)]
impl<T: Ord + Display> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: Option::None }
    }

    pub fn inorder_traverse(&self) {
        match &self.root {
            Some(node) => node.inorder_traverse(),
            None => println!("tree is empty"),
        }
    }

    pub fn insert(&mut self, value: T) {
        match &mut self.root {
            Some(node) => node.insert(value),
            None => {
                self.root = Some(Box::new(TreeNode {
                    data: value,
                    left: None,
                    right: None,
                }))
            }
        }
    }

    pub fn search(&self, _: T) {
        match &self.root {
            Some(node) => println!("{}", node.data),
            None => println!("tree is empty"),
        }
    }
}

pub fn binary_tree_test() {
    let root = TreeNode {
        data: 5,
        left: Some(Box::new(TreeNode {
            data: 3,
            left: Some(Box::new(TreeNode {
                data: 1,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                data: 4,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            data: 8,
            left: Some(Box::new(TreeNode {
                data: 7,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                data: 10,
                left: None,
                right: None,
            })),
        })),
    };

    let btree = BinarySearchTree::<i32> {
        root: Some(Box::new(root)),
    };

    btree.inorder_traverse();
}

pub fn binary_tree_create_test() {
    let mut bst = BinarySearchTree::new();

    // Insert values
    bst.insert(5);
    bst.insert(3);
    bst.insert(8);
    bst.insert(1);
    bst.insert(4);
    bst.insert(7);
    bst.insert(10);

    println!("In-order traversal (sorted):");
    bst.inorder_traverse();
}
