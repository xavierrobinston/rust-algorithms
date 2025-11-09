#![allow(unused)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

type WeakNode = Weak<Node>;
type RcNode = Rc<Node>;

#[derive(Debug)]
pub struct Node {
    value: i32,
    parent: RefCell<WeakNode>,
    children: RefCell<Vec<RcNode>>,
}

pub fn test_weak_circular_ref() {
    let child = Rc::new(Node {
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let parent = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&child)]),
    });

    // program crashes when we enable the following line
    *child.parent.borrow_mut() = Rc::downgrade(&parent);

    println!("Parent: {:#?}", parent);
    println!("Children: {:#?}", child)
}
