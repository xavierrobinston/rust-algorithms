#![allow(unused)]
#![allow(dead_code)]

use std::{default, fmt::Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ListNode {
    pub value: i32,
    pub prev: Option<Box<ListNode>>,
    pub next: Option<Box<ListNode>>,
}

#[derive(Debug)]
pub struct LinkedList {
    pub head: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(value: i32) -> Self {
        ListNode {
            value: value,
            prev: Option::None,
            next: Option::None,
        }
    }
}
impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: Option::None }
    }

    pub fn empty(&self) -> bool {
        match &self.head {
            None => false,
            default => true,
        }
    }

    pub fn add_head(&mut self, value: i32) {
        match self.head.take() {
            None => self.head = Some(Box::new(ListNode::new(value))),
            Some(item) => {
                let new_head = Box::new(ListNode {
                    value: value,
                    next: Some(item),
                    prev: None,
                });
                self.head = Some(new_head);
            }
        }
    }

    pub fn remove_head(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(item) => {
                self.head = item.next;
                Some(item.value)
            }
        }
    }

    pub fn peek_head(&self) -> Option<i32> {
        match &self.head {
            None => None,
            Some(item) => Some(item.value),
        }
    }
}

pub fn linkedlist_create_test() {
    let mut list = LinkedList::new();

    // Add nodes to the head
    list.add_head(10);
    list.add_head(20);
    list.add_head(30);

    println!("{:#?}", list);

    let mut head_value = list.remove_head().unwrap_or(-1);
    println!("{:#?}", head_value);

    println!("After head removed: {:#?}", list);

    head_value = list.peek_head().unwrap_or(-1);
    println!("{:#?}", head_value);

    println!("After head peeked: {:#?}", list);
}
