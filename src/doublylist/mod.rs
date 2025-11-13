#![allow(unused)]
#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::{default, fmt::Display};

#[derive(Debug)]
pub struct DNode {
    value: i32,
    next: Option<Rc<RefCell<DNode>>>,
    prev: Option<Weak<RefCell<DNode>>>,
}

#[derive(Debug)]
pub struct DoublyList {
    pub head: Option<Rc<RefCell<DNode>>>,
    pub tail: Option<Weak<RefCell<DNode>>>,
}

impl DNode {
    pub fn new(value: i32) -> Self {
        DNode {
            value: value,
            prev: None,
            next: None,
        }
    }
}
impl DoublyList {
    pub fn new() -> Self {
        DoublyList {
            head: None,
            tail: None,
        }
    }

    pub fn empty(&self) -> bool {
        match &self.head {
            None => false,
            default => true,
        }
    }

    pub fn add_head(&mut self, value: i32) {
        match self.head.take() {
            // make the existing head next item of new node
            Some(item) => {
                let new_node = Rc::new(RefCell::new(DNode {
                    value: value,
                    next: Some(item.clone()),
                    prev: None,
                }));

                // set the existing head's prev to new_node
                let mut m = item.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_node));

                // set the new_node
                self.head = Some(new_node);
            }

            None => {
                let new_node = Rc::new(RefCell::new(DNode::new(value)));

                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }
    }

    pub fn add_tail(&mut self, value: i32) {
        match self.tail.take() {
            // make the existing htailead next item of new node
            Some(item) => {
                let new_node = Rc::new(RefCell::new(DNode {
                    value: value,
                    prev: Some(item.clone()),
                    next: None,
                }));

                // set the existing htailead's prev to new_node
                let st = Weak::upgrade(&item).unwrap();
                let mut m = st.borrow_mut();
                self.tail = Some(Rc::downgrade(&new_node));

                m.next = Some(new_node);

                // set the new_node
            }

            None => {
                let new_node = Rc::new(RefCell::new(DNode::new(value)));

                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }
    }
}

pub fn test_create_doublylist() {
    let mut list = DoublyList::new();

    list.add_head(6);
    list.add_tail(11);
    list.add_head(5);
    list.add_head(4);
    list.add_head(3);
    list.add_tail(15);

    println!("{:#?}", list);
}
