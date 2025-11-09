#![allow(unused)]
#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ListNode<T> {
    pub data: T,
    pub prev: Option<Box<ListNode<T>>>,
    pub next: Option<Box<ListNode<T>>>,
}

#[derive(Debug)]
pub struct BinarySearchTree<T> {
    pub root: Option<Box<ListNode<T>>>,
}

impl<T: Ord + Display> ListNode<T> {
    pub fn inorder_traverse(&self) {
        if let Some(node) = &self.prev {
            node.inorder_traverse()
        }

        println!("{}", self.data);

        if let Some(node) = &self.next {
            node.inorder_traverse()
        }
    }

    pub fn insert_after(&mut self, value: T) {
        if value < self.data {
            match &mut self.prev {
                Some(node) => node.insert_after(value),
                None => {
                    self.prev = Some(Box::new(ListNode {
                        data: value,
                        prev: None,
                        next: None,
                    }))
                }
            }
        } else if value > self.data {
            match &mut self.next {
                Some(node) => node.insert_after(value),
                None => {
                    self.next = Some(Box::new(ListNode {
                        data: value,
                        prev: None,
                        next: None,
                    }))
                }
            }
        }
    }

    pub fn search(&self, value: T) -> Option<&ListNode<T>> {
        if value < self.data {
            match &self.prev {
                Some(prev) => prev.search(value),
                None => None,
            }
        } else if value > self.data {
            match &self.next {
                Some(next) => next.search(value),
                None => None,
            }
        } else {
            return Some(self);
        }
    }
}

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
            Some(node) => node.insert_after(value),
            None => {
                self.root = Some(Box::new(ListNode {
                    data: value,
                    prev: None,
                    next: None,
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

pub fn linked_test() {
    let root = ListNode {
        data: 5,
        prev: Some(Box::new(ListNode {
            data: 3,
            prev: Some(Box::new(ListNode {
                data: 1,
                prev: None,
                next: None,
            })),
            next: Some(Box::new(ListNode {
                data: 4,
                prev: None,
                next: None,
            })),
        })),
        next: Some(Box::new(ListNode {
            data: 8,
            prev: Some(Box::new(ListNode {
                data: 7,
                prev: None,
                next: None,
            })),
            next: Some(Box::new(ListNode {
                data: 10,
                prev: None,
                next: None,
            })),
        })),
    };

    let btree = BinarySearchTree::<i32> {
        root: Some(Box::new(root)),
    };

    btree.inorder_traverse();
}

pub fn linkedlist_create_test() {
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
